pub fn lookup<E: std::str::FromStr>(name: &str, default: E) -> E {
    if let Ok(var) = std::env::var(name) {
        var.parse::<E>().unwrap_or(default)
    } else {
        default
    }
}
