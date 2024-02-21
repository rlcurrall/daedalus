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
    static MAILBOX: Mailbox;
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Mailbox {
    pub(self) messages: RefCell<HashMap<String, String>>,
    pub(self) errors: RefCell<HashMap<String, String>>,
    pub(self) data: RefCell<HashMap<String, String>>,
}

impl Mailbox {
    pub(self) fn new() -> Self {
        Self {
            messages: RefCell::new(HashMap::new()),
            errors: RefCell::new(HashMap::new()),
            data: RefCell::new(HashMap::new()),
        }
    }

    pub(self) fn is_empty(&self) -> bool {
        self.messages.borrow().is_empty() && self.errors.borrow().is_empty()
    }
}

impl Default for Mailbox {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FlashMessagesMiddleware<S> {
    service: S,
    storage_backend: Arc<dyn FlashMessageStore>,
}

impl<S, B> Service<ServiceRequest> for FlashMessagesMiddleware<S>
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
        req.extensions_mut().insert(self.storage_backend.clone());
        let mailbox = Mailbox::new();
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
                    .unwrap();
                response
            })
        }))
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
enum FlashType {
    Message,
    Error,
    Data,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Flash {
    key: String,
    content: String,
    flash_type: FlashType,
}

impl Flash {
    pub fn message(key: String, content: String) -> Result<(), AppError> {
        MAILBOX
            .try_with(|mailbox| {
                mailbox.messages.borrow_mut().insert(key, content);
            })
            .map_err(|e| AppError::server_error(e))
    }

    pub fn error(key: String, content: String) -> Result<(), AppError> {
        MAILBOX
            .try_with(|mailbox| {
                mailbox.errors.borrow_mut().insert(key, content);
            })
            .map_err(|e| AppError::server_error(e))
    }

    pub fn data(key: String, content: String) -> Result<(), AppError> {
        MAILBOX
            .try_with(|mailbox| {
                mailbox.data.borrow_mut().insert(key, content);
            })
            .map_err(|e| AppError::server_error(e))
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct IncomingFlashMessages {
    messages: HashMap<String, String>,
    errors: HashMap<String, String>,
    data: HashMap<String, String>,
}

impl IncomingFlashMessages {
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

impl FromRequest for IncomingFlashMessages {
    type Error = actix_web::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        std::future::ready(extract_flash_messages(req))
    }
}

fn extract_flash_messages(req: &HttpRequest) -> Result<IncomingFlashMessages, actix_web::Error> {
    let message_store = req.extensions()
        .get::<Arc<dyn FlashMessageStore>>()
        .ok_or(AppError::server_error("Failed to retrieve flash messages!\n\
            To use the `IncomingFlashMessages` extractor you need to add `FlashMessageFramework` as a middleware \
            on your `actix-web` application using `wrap`. Check out `actix-web-flash-messages`'s documentation for more details."))?
        // Cloning here is necessary in order to drop our reference to the request extensions.
        // Some of the methods on `req` will in turn try to use `req.extensions_mut()`, leading to a borrow
        // panic at runtime due to the usage of interior mutability.
        .to_owned();

    message_store
        .load(req)
        .map(|m| IncomingFlashMessages {
            messages: m.messages.into_inner(),
            errors: m.errors.into_inner(),
            data: m.data.into_inner(),
        })
        .map_err(|e| actix_web::error::InternalError::new(e, StatusCode::BAD_REQUEST).into())
}

pub trait FlashMessageStore: Send + Sync {
    fn load(&self, request: &HttpRequest) -> Result<Mailbox, AppError>;

    fn store(
        &self,
        mailbox: &Mailbox,
        request: HttpRequest,
        response: &mut ResponseHead,
    ) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct SessionMessageStore {
    key: String,
}

impl SessionMessageStore {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Default for SessionMessageStore {
    fn default() -> Self {
        Self {
            key: "_flash".into(),
        }
    }
}

impl FlashMessageStore for SessionMessageStore {
    fn load(&self, request: &HttpRequest) -> Result<Mailbox, AppError> {
        let session = request.get_session();
        let mailbox = session
            .get(&self.key)
            .map_err(|e| AppError::server_error(e))?
            .unwrap_or_default();
        Ok(mailbox)
    }

    fn store(
        &self,
        mailbox: &Mailbox,
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
pub struct FlashMiddlewareBuilder {
    pub(crate) storage_backend: Arc<dyn FlashMessageStore>,
}

impl FlashMiddlewareBuilder {
    pub fn new<S: FlashMessageStore + 'static>(storage_backend: S) -> Self {
        Self {
            storage_backend: Arc::new(storage_backend),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for FlashMiddlewareBuilder
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = FlashMessagesMiddleware<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(FlashMessagesMiddleware {
            service,
            storage_backend: self.storage_backend.clone(),
        }))
    }
}
