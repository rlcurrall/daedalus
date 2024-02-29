use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

use rust_embed::RustEmbed;
pub use tera::Context;
use tera::Tera;

use crate::result::{AppError, Result};

#[derive(RustEmbed)]
#[folder = "resources/templates"]
struct TemplateFiles;

#[derive(Clone)]
pub struct Tmpl {
    dev: bool,
    templates: Arc<Mutex<Tera>>,
}

impl Tmpl {
    pub fn init(version: String, dev: bool) -> Result<Self> {
        let mut templates = Tera::default();

        templates.register_function("version", InjectVersion::new(version.clone()));
        templates.register_function("asset_path", InjectAssetPath::new(version.clone()));
        templates.register_function("vite", InjectVite);
        templates.register_function("js", InjectJs);
        templates.register_function("css", InjectCss);

        let instance = Self {
            dev,
            templates: Arc::new(Mutex::new(templates)),
        };

        instance.reload()?;

        Ok(instance)
    }

    pub fn render(&self, name: &str, context: &tera::Context) -> Result<String> {
        if self.dev {
            self.reload()?;
        }

        self.templates
            .lock()
            .map_err(|e| AppError::ServerError {
                cause: format!("Failed to render view: {}", e),
            })?
            .render(name, context)
            .map_err(|e| {
                let cause = match e.source() {
                    Some(source) => format!("{}, {}", e, source),
                    None => e.to_string(),
                };
                tracing::error!("{cause}");
                AppError::ServerError { cause }
            })
    }

    pub fn reload(&self) -> Result<()> {
        let mut tera = self.templates.lock().map_err(|e| AppError::ServerError {
            cause: format!("Failed to reload templates: {}", e),
        })?;

        tera.add_raw_templates(
            TemplateFiles::iter()
                .filter_map(|file| {
                    let name = file.as_ref();
                    let content = TemplateFiles::get(name)?;
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

struct InjectVite;

impl tera::Function for InjectVite {
    fn call(&self, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        Ok(tera::Value::String(format!(
            r#"<script type="module" src="http://localhost:5173/@vite/client"></script>"#
        )))
    }
}

struct InjectJs;

impl tera::Function for InjectJs {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let bundle_name = args.get("name").and_then(tera::Value::as_str).unwrap_or("");
        let inject = format!(
            r#"<script type="module" src="http://localhost:5173/resources/js/{bundle_name}"></script>"#
        );
        Ok(tera::Value::String(inject))
    }
}

struct InjectCss;

impl tera::Function for InjectCss {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let bundle_name = args.get("name").and_then(tera::Value::as_str).unwrap_or("");
        let inject = format!(
            r#"<link rel="stylesheet" href="http://localhost:5173/resources/css/{bundle_name}" />"#
        );
        Ok(tera::Value::String(inject))
    }
}

struct InjectVersion {
    version: String,
}

impl InjectVersion {
    pub fn new(version: String) -> Self {
        Self { version }
    }
}

impl tera::Function for InjectVersion {
    fn call(&self, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        Ok(tera::Value::String(self.version.clone()))
    }
}

struct InjectAssetPath {
    version: String,
}

impl InjectAssetPath {
    pub fn new(version: String) -> Self {
        Self { version }
    }
}

impl tera::Function for InjectAssetPath {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let name = args.get("name").and_then(tera::Value::as_str).unwrap_or("");
        let version = &self.version;
        let path = format!("/{}/{}", version, name);
        Ok(tera::Value::String(path))
    }
}
