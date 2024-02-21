use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::{cell::RefCell, collections::HashMap};

use actix_session::SessionExt;
use actix_web::body::MessageBody;
use actix_web::dev::{ResponseHead, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::StatusCode;
use actix_web::{FromRequest, HttpMessage, HttpRequest};

use crate::result::AppError;

tokio::task_local! {
    static MAILBOX: FlashMailbox;
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct FlashMailbox {
    pub(self) messages: RefCell<HashMap<String, String>>,
    pub(self) errors: RefCell<HashMap<String, String>>,
    pub(self) data: RefCell<HashMap<String, String>>,
}

impl FlashMailbox {
    pub(self) fn new() -> Self {
        Self {
            messages: RefCell::new(HashMap::new()),
            errors: RefCell::new(HashMap::new()),
            data: RefCell::new(HashMap::new()),
        }
    }

    pub(self) fn is_empty(&self) -> bool {
        self.messages.borrow().is_empty()
            && self.errors.borrow().is_empty()
            && self.data.borrow().is_empty()
    }
}

impl Default for FlashMailbox {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FlashTransform<S> {
    service: S,
    storage_backend: Arc<dyn FlashStore>,
}

impl<S, B> Service<ServiceRequest> for FlashTransform<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let mailbox = FlashMailbox::new();
        req.extensions_mut().insert(self.storage_backend.clone());
        // Working with task-locals in actix-web middlewares is a bit annoying.
        // We need to make the task local value available to the rest of the middleware chain, which
        // generates the `future` which will in turn return us a response.
        // This generation process is synchronous, so we must use `sync_scope`.
        let future = MAILBOX.sync_scope(mailbox.clone(), move || self.service.call(req));
        // We can then make the task local value available to the asynchronous execution context
        // using `scope` without losing the messages that might have been recorded by the middleware
        // chain.
        let storage_backend = self.storage_backend.clone();
        Box::pin(MAILBOX.scope(mailbox, async move {
            let response: Result<Self::Response, Self::Error> = future.await;
            response.map(|mut response| {
                MAILBOX
                    .with(|m| {
                        storage_backend.store(
                            &m,
                            // This `.clone()` is cheap because `HttpRequest` is just an `Rc` pointer
                            // around the actual request data.
                            response.request().clone(),
                            response.response_mut().head_mut(),
                        )
                    })
                    .map_err(|e| {
                        tracing::error!("Failed to store flash messages: {}", e);
                    })
                    .ok();
                response
            })
        }))
    }
}

pub struct Flash;

impl Flash {
    pub fn message(key: &str, content: String) -> Result<(), AppError> {
        MAILBOX
            .try_with(|mailbox| {
                mailbox.messages.borrow_mut().insert(key.into(), content);
            })
            .map_err(|e| AppError::server_error(e))
    }

    pub fn error(key: &str, content: String) -> Result<(), AppError> {
        MAILBOX
            .try_with(|mailbox| {
                mailbox.errors.borrow_mut().insert(key.into(), content);
            })
            .map_err(|e| AppError::server_error(e))
    }

    pub fn data(key: &str, content: String) -> Result<(), AppError> {
        MAILBOX
            .try_with(|mailbox| {
                mailbox.data.borrow_mut().insert(key.into(), content);
            })
            .map_err(|e| AppError::server_error(e))
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct FlashInbox {
    messages: HashMap<String, String>,
    errors: HashMap<String, String>,
    data: HashMap<String, String>,
}

impl FlashInbox {
    pub fn messages(&self) -> &HashMap<String, String> {
        &self.messages
    }

    pub fn errors(&self) -> &HashMap<String, String> {
        &self.errors
    }

    pub fn data(&self) -> &HashMap<String, String> {
        &self.data
    }
}

impl FromRequest for FlashInbox {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        std::future::ready(extract_flash_messages(req))
    }
}

fn extract_flash_messages(req: &HttpRequest) -> Result<FlashInbox, actix_web::Error> {
    let message_store = req.extensions()
        .get::<Arc<dyn FlashStore>>()
        .ok_or(AppError::server_error("Failed to retrieve flash messages!\n\
            To use the `IncomingFlashes` extractor you need to add `FlashMiddleware` as a middleware \
            on your `actix-web` application using `wrap`. Check out `actix-web-flash-messages`'s documentation for more details."))?
        // Cloning here is necessary in order to drop our reference to the request extensions.
        // Some of the methods on `req` will in turn try to use `req.extensions_mut()`, leading to a borrow
        // panic at runtime due to the usage of interior mutability.
        .to_owned();

    message_store
        .load(req)
        .map(|m| FlashInbox {
            messages: m.messages.into_inner(),
            errors: m.errors.into_inner(),
            data: m.data.into_inner(),
        })
        .map_err(|e| actix_web::error::InternalError::new(e, StatusCode::BAD_REQUEST).into())
}

pub trait FlashStore: Send + Sync {
    fn load(&self, request: &HttpRequest) -> Result<FlashMailbox, AppError>;

    fn store(
        &self,
        mailbox: &FlashMailbox,
        request: HttpRequest,
        response: &mut ResponseHead,
    ) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct SessionStore {
    key: String,
}

impl SessionStore {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Default for SessionStore {
    fn default() -> Self {
        Self {
            key: "_flash".into(),
        }
    }
}

impl FlashStore for SessionStore {
    fn load(&self, request: &HttpRequest) -> Result<FlashMailbox, AppError> {
        let session = request.get_session();
        let mailbox = session
            .get(&self.key)
            .map_err(|e| AppError::server_error(e))?
            .unwrap_or_default();
        Ok(mailbox)
    }

    fn store(
        &self,
        mailbox: &FlashMailbox,
        request: HttpRequest,
        _response: &mut ResponseHead,
    ) -> Result<(), AppError> {
        let session = request.get_session();
        if mailbox.is_empty() {
            // Make sure to clear up previous flash messages
            // No need to do this on the other if-branch because we are overwriting
            // any pre-existing flash message with a new value.
            session.remove(&self.key);
        } else {
            session
                .insert(&self.key, mailbox)
                .map_err(|e| AppError::server_error(e))?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct FlashMiddleware {
    pub(self) storage_backend: Arc<dyn FlashStore>,
}

impl FlashMiddleware {
    pub fn new<S: FlashStore + 'static>(storage_backend: S) -> Self {
        Self {
            storage_backend: Arc::new(storage_backend),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for FlashMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = FlashTransform<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(FlashTransform {
            service,
            storage_backend: self.storage_backend.clone(),
        }))
    }
}
