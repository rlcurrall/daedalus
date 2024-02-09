use std::sync::Mutex;

pub use tera::Context;
use tera::Tera;

use crate::result::{AppError, Result};

pub struct View {
    views: Mutex<Tera>,
}

impl View {
    pub fn init() -> Result<Self> {
        let tera = Tera::new("views/**/*.njk").map_err(|e| AppError::ServerError {
            cause: e.to_string(),
        })?;

        Ok(Self {
            views: Mutex::new(tera),
        })
    }

    pub fn render(&self, name: &str, context: &tera::Context) -> Result<String> {
        self.views
            .lock()
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to render view: {}", e),
            })?
            .render(name, context)
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to render view: {}", e),
            })
    }

    pub fn one_off(&self, template: &str, context: &tera::Context) -> Result<String> {
        self.views
            .lock()
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to render view: {}", e),
            })?
            .render_str(template, context)
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to render view: {}", e),
            })
    }

    pub fn reload(&self) -> Result<()> {
        self.views
            .lock()
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to reload views: {}", e),
            })?
            .full_reload()
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to reload views: {}", e),
            })
    }
}
