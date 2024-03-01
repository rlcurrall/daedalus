use std::collections::HashMap;

use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct Manifest {
    pub(self) inner: HashMap<String, ManifestEntry>,
}

impl Manifest {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        let inner = serde_json::from_slice(bytes)?;
        Ok(Self { inner })
    }
}

#[derive(Clone, Debug, Deserialize)]
struct ManifestEntry {
    pub(self) file: String,
}

pub struct InjectVite {
    dev: bool,
}

impl InjectVite {
    pub const KEY: &'static str = "vite";

    pub fn new() -> Self {
        Self { dev: true }
    }

    pub fn set_dev(mut self, dev: bool) -> Self {
        self.dev = dev;
        self
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

pub struct InjectJs {
    dev: bool,
    manifest: Manifest,
    dev_path: String,
    build_path: String,
}

impl InjectJs {
    pub const KEY: &'static str = "js";

    pub fn new() -> Self {
        Self {
            dev: true,
            manifest: Manifest::new(),
            dev_path: "resources/js".to_string(),
            build_path: "build".to_string(),
        }
    }

    pub fn set_dev(mut self, dev: bool) -> Self {
        self.dev = dev;
        self
    }

    pub fn set_manifest(mut self, manifest: Manifest) -> Self {
        self.manifest = manifest;
        self
    }

    pub fn set_dev_path(mut self, dev_path: impl ToString) -> Self {
        self.dev_path = dev_path.to_string();
        self
    }

    #[allow(dead_code)]
    pub fn set_build_path(mut self, build_path: impl ToString) -> Self {
        self.build_path = build_path.to_string();
        self
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
                r#"<script type="module" src="http://localhost:5173/{}/{}"></script>"#,
                self.dev_path, name
            );
            return Ok(tera::Value::String(inject));
        }

        let manifest_id = format!("{}/{}", self.dev_path, name);
        let manifest = self.manifest.inner.get(&manifest_id);
        match manifest {
            None => Err(tera::Error::msg(format!(
                "Failed to find js bundle: {name}"
            ))),
            Some(manifest) => {
                let inject = format!(
                    r#"<script type="module" src="/{}/{}"></script>"#,
                    self.build_path, manifest.file
                );
                Ok(tera::Value::String(inject))
            }
        }
    }
}

pub struct InjectCss {
    dev: bool,
    manifest: Manifest,
    dev_path: String,
    build_path: String,
}

impl InjectCss {
    pub const KEY: &'static str = "css";

    pub fn new() -> Self {
        Self {
            dev: true,
            manifest: Manifest::new(),
            dev_path: "resources/css".to_string(),
            build_path: "build".to_string(),
        }
    }

    pub fn set_dev(mut self, dev: bool) -> Self {
        self.dev = dev;
        self
    }

    pub fn set_manifest(mut self, manifest: Manifest) -> Self {
        self.manifest = manifest;
        self
    }

    #[allow(dead_code)]
    pub fn set_dev_path(mut self, dev_path: impl ToString) -> Self {
        self.dev_path = dev_path.to_string();
        self
    }

    #[allow(dead_code)]
    pub fn set_build_path(mut self, build_path: impl ToString) -> Self {
        self.build_path = build_path.to_string();
        self
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
                r#"<link rel="stylesheet" href="http://localhost:5173/{}/{}" />"#,
                self.dev_path, bundle_name
            );
            return Ok(tera::Value::String(inject));
        }

        let manifest_id = format!("resources/css/{bundle_name}");
        let manifest = self.manifest.inner.get(&manifest_id);
        match manifest {
            None => Err(tera::Error::msg(format!(
                "Failed to find css bundle: {bundle_name}"
            ))),
            Some(manifest) => {
                let inject = format!(
                    r#"<link rel="stylesheet" href="/{}/{}" />"#,
                    self.build_path, manifest.file
                );
                Ok(tera::Value::String(inject))
            }
        }
    }
}

pub struct InjectReactRefresh {
    dev: bool,
}

impl InjectReactRefresh {
    pub const KEY: &'static str = "react_refresh";

    pub fn new() -> Self {
        Self { dev: true }
    }

    pub fn set_dev(mut self, dev: bool) -> Self {
        self.dev = dev;
        self
    }
}

impl tera::Function for InjectReactRefresh {
    fn call(&self, _: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        if self.dev {
            Ok(tera::Value::String(
                r#"
                <script type="module">
                    import RefreshRuntime from 'http://localhost:5173/@react-refresh'
                    RefreshRuntime.injectIntoGlobalHook(window)
                    window.$RefreshReg$ = () => {{ /* noop */ }}
                    window.$RefreshSig$ = () => (type) => type
                    window.__vite_plugin_react_preamble_installed__ = true
                </script>
                "#
                .to_string(),
            ))
        } else {
            Ok(tera::Value::Null)
        }
    }
}
