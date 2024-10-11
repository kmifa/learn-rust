use std::env;
use strum::EnumString;

#[derive(Default, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Enviroment {
    #[default]
    Development,
    Production,
}

pub fn which() -> Enviroment {
    #[cfg(debug_assertions)]
    let default_env = Enviroment::Development;
    #[cfg(not(debug_assertions))]
    let default_env = Enviroment::Production;

    match env::var("ENV") {
        Err(_) => default_env,
        Ok(v) => v.parse().unwrap_or(default_env),
    }
}
