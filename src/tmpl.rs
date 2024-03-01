use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

pub use tera::Context;
use tera::Tera;

use crate::embedded::{PublicFiles, TemplateFiles};
use crate::result::{AppError, Result};

use self::vite::{InjectCss, InjectJs, InjectReactRefresh, InjectVite, Manifest};

mod vite;

#[derive(Clone)]
pub struct Tmpl {
    dev: bool,
    templates: Arc<Mutex<Tera>>,
}

impl Tmpl {
    pub fn init(version: String, dev: bool) -> Result<Self> {
        let manifest = match dev {
            true => Manifest::new(),
            false => Self::load_manifest()?,
        };

        let mut templates = Tera::default();
        templates.register_function("version", InjectVersion::new(version.clone()));
        templates.register_function(InjectVite::KEY, InjectVite::new().set_dev(dev));
        templates.register_function(
            InjectJs::KEY,
            InjectJs::new()
                .set_dev(dev)
                .set_manifest(manifest.clone())
                .set_dev_path("resources/ts"),
        );
        templates.register_function(
            InjectCss::KEY,
            InjectCss::new().set_dev(dev).set_manifest(manifest.clone()),
        );
        templates.register_function(
            InjectReactRefresh::KEY,
            InjectReactRefresh::new().set_dev(dev),
        );
        let templates = Arc::new(Mutex::new(templates));

        let instance = Self { dev, templates };

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

    fn load_manifest() -> Result<Manifest> {
        PublicFiles::get("build/manifest.json")
            .ok_or(AppError::server_error("Failed to load manifest.json"))
            .and_then(|f| Manifest::from_bytes(&f.data).map_err(|e| AppError::server_error(e)))
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
