
#[derive(serde::Deserialize,Debug)]
pub struct Settings {
    pub http_url: String, 
    pub ws_url: Option<String>,
}

pub fn new(path: &str) -> Settings {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            path,//"configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build();
    settings.unwrap().try_deserialize().unwrap()        
}

pub fn get(path: &str) -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            path,//"configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}