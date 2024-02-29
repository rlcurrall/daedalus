use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

use serde::Deserialize;
pub use tera::Context;
use tera::Tera;

use crate::embedded::{PublicFiles, TemplateFiles};
use crate::result::{AppError, Result};

#[derive(Clone)]
pub struct Tmpl {
    dev: bool,
    templates: Arc<Mutex<Tera>>,
}

#[derive(Clone, Debug, Deserialize)]
struct Manifest {
    pub(self) file: String,
}

impl Tmpl {
    pub fn init(version: String, dev: bool) -> Result<Self> {
        let manifest = if dev {
            HashMap::new()
        } else {
            Self::load_manifest()?
        };

        let mut templates = Tera::default();
        templates.register_function("version", InjectVersion::new(version.clone()));
        templates.register_function("vite", InjectVite::new(dev));
        templates.register_function("js", InjectJs::new(dev, manifest.clone()));
        templates.register_function("css", InjectCss::new(dev, manifest.clone()));
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

    fn load_manifest() -> Result<HashMap<String, Manifest>> {
        PublicFiles::get("build/manifest.json")
            .ok_or(AppError::server_error("Failed to load manifest.json"))
            .and_then(|f| {
                String::from_utf8(f.data.into_owned()).map_err(|e| AppError::server_error(e))
            })
            .and_then(|c| {
                serde_json::from_str::<HashMap<String, Manifest>>(&c)
                    .map_err(|e| AppError::server_error(e))
            })
    }
}

struct InjectVite {
    dev: bool,
}

impl InjectVite {
    pub fn new(dev: bool) -> Self {
        Self { dev }
    }
}

impl tera::Function for InjectVite {
    fn call(&self, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        if self.dev {
            Ok(tera::Value::String(
                r#"<script type="module" src="http://localhost:5173/@vite/client"></script>"#
                    .to_string(),
            ))
        } else {
            Ok(tera::Value::Null)
        }
    }
}

struct InjectJs {
    dev: bool,
    manifest: HashMap<String, Manifest>,
}

impl InjectJs {
    pub fn new(dev: bool, manifest: HashMap<String, Manifest>) -> Self {
        Self { dev, manifest }
    }
}

impl tera::Function for InjectJs {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let name = args
            .get("name")
            .and_then(tera::Value::as_str)
            .ok_or(tera::Error::msg(
                "No name provided for js bundle. Example: {{ js('app.js') }}",
            ))?;

        if self.dev {
            let inject = format!(
                r#"<script type="module" src="http://localhost:5173/resources/js/{name}"></script>"#
            );
            return Ok(tera::Value::String(inject));
        }

        let manifest_id = format!("resources/js/{name}");
        let manifest = self.manifest.get(&manifest_id);
        match manifest {
            None => Err(tera::Error::msg(format!(
                "Failed to find js bundle: {name}"
            ))),
            Some(manifest) => {
                let inject = format!(
                    r#"<script type="module" src="/build/{}"></script>"#,
                    manifest.file
                );
                Ok(tera::Value::String(inject))
            }
        }
    }
}

struct InjectCss {
    dev: bool,
    manifest: HashMap<String, Manifest>,
}

impl InjectCss {
    pub fn new(dev: bool, manifest: HashMap<String, Manifest>) -> Self {
        Self { dev, manifest }
    }
}

impl tera::Function for InjectCss {
    fn call(&self, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let bundle_name =
            args.get("name")
                .and_then(tera::Value::as_str)
                .ok_or(tera::Error::msg(
                    "No name provided for css bundle. Example: {{ css('app.css') }}",
                ))?;

        if self.dev {
            let inject = format!(
                r#"<link rel="stylesheet" href="http://localhost:5173/resources/css/{bundle_name}" />"#
            );
            return Ok(tera::Value::String(inject));
        }

        let manifest_id = format!("resources/css/{bundle_name}");
        let manifest = self.manifest.get(&manifest_id);
        match manifest {
            None => Err(tera::Error::msg(format!(
                "Failed to find css bundle: {bundle_name}"
            ))),
            Some(manifest) => {
                let inject = format!(
                    r#"<link rel="stylesheet" href="/build/{}" />"#,
                    manifest.file
                );
                Ok(tera::Value::String(inject))
            }
        }
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
