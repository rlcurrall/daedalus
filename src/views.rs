use std::sync::Mutex;

use once_cell::sync::OnceCell;
pub use tera::Context;
use tera::Tera;

use crate::result::{AppError, Result};

static VIEWS: OnceCell<Mutex<Tera>> = OnceCell::new();

pub struct View;

impl View {
    pub fn init() -> Result<()> {
        if VIEWS.get().is_some() {
            return Err(AppError::ServerError {
                cause: "Views already initialized".to_string(),
            });
        }

        let tera = Tera::new("views/**/*.njk").map_err(|e| AppError::ServerError {
            cause: e.to_string(),
        })?;

        VIEWS
            .set(Mutex::new(tera))
            .map_err(|_| AppError::ServerError {
                cause: "Could not initialize views".to_string(),
            })?;

        Ok(())
    }

    pub fn render(name: &str, context: &tera::Context) -> Result<String> {
        VIEWS
            .get()
            .ok_or(AppError::ServerError {
                cause: format!("Views not initialized"),
            })?
            .lock()
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to render view: {}", e),
            })?
            .render(name, context)
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to render view: {}", e),
            })
    }

    pub fn reload() -> Result<()> {
        VIEWS
            .get()
            .ok_or(AppError::ServerError {
                cause: format!("Views not initialized"),
            })?
            .lock()
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to render view: {}", e),
            })?
            .full_reload()
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to reload views: {}", e),
            })
    }
}
