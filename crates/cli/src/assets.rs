use once_cell::sync::Lazy;
use std::collections::HashMap;

pub(crate) static ASSETS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("project.sql", include_str!("../assets/project.sql"));
    m.insert("PARTS.csv", include_str!("../assets/PARTS.csv"));
    m.insert("PRICE.csv", include_str!("../assets/PRICE.csv"));
    m.insert("VENDOR.csv", include_str!("../assets/VENDOR.csv"));
    m.insert("Cargo.toml", include_str!("../assets/Cargo.toml"));
    m.insert("main.rs", include_str!("../assets/main.rs"));
    m.insert("config.json", include_str!("../assets/config.json"));
    m
});
