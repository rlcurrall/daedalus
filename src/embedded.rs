use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "resources/templates"]
pub struct TemplateFiles;

#[derive(RustEmbed)]
#[folder = "public"]
pub struct PublicFiles;
