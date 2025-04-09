use std::{ env, str };

pub fn lookup<E: str::FromStr>(name: &str, default: E) -> E {
    if let Ok(var) = env::var(name) { var.parse::<E>().unwrap_or(default) } else { default }
}
