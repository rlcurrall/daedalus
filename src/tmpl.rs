use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use rust_embed::RustEmbed;
use serde_json::Value;
pub use tera::Context;
use tera::Tera;

use crate::result::{AppError, Result};

#[derive(RustEmbed)]
#[folder = "templates"]
struct Assets;

#[derive(Clone)]
pub struct Tmpl {
    templates: Arc<Mutex<Tera>>,
}

impl Tmpl {
    pub fn init(version: String) -> Result<Self> {
        let mut templates = Tera::default();

        let version_c1 = version.clone();
        templates.register_function("version", move |_: &HashMap<String, Value>| {
            Ok(Value::String(version_c1.clone()))
        });

        templates.register_function("asset_path", move |args: &HashMap<String, Value>| {
            let name = args.get("name").and_then(Value::as_str).unwrap_or("");
            let version = version.clone();
            let path = format!("/{}/{}", version, name);
            Ok(Value::String(path))
        });

        let instance = Self {
            templates: Arc::new(Mutex::new(templates)),
        };

        instance.reload()?;

        Ok(instance)
    }

    pub fn render(&self, name: &str, context: &tera::Context) -> Result<String> {
        self.templates
            .lock()
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to render view: {}", e),
            })?
            .render(name, context)
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to render view: {}", e),
            })
    }

    pub fn reload(&self) -> Result<()> {
        let mut tera = self.templates.lock().map_err(|e| AppError::ServerError {
            cause: format!("Failed to reload templates: {}", e),
        })?;

        tera.add_raw_templates(
            Assets::iter()
                .filter_map(|file| {
                    let name = file.as_ref();
                    let content = Assets::get(name)?;
                    let content = String::from_utf8(content.data.into_owned()).ok()?;
                    Some((name.to_string(), content))
                })
                .collect::<Vec<_>>(),
        )
        .map_err(|e| AppError::ServerError {
            cause: format!("Failed to load templates: {}", e),
        })?;

        Ok(())
    }
}
