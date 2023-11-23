use once_cell::sync::Lazy;
use std::collections::HashMap;

pub(crate) static ASSETS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("project.sql", include_str!("project.sql"));
    m.insert("PART.csv", include_str!("PART.csv"));
    m.insert("PRICE.csv", include_str!("PRICE.csv"));
    m.insert("VENDOR.csv", include_str!("VENDOR.csv"));
    m.insert("Cargo.toml", include_str!("Cargo.toml"));
    m.insert("main.rs", include_str!("main.rs"));
    m.insert("config.json", include_str!("config.json"));
    m
});

pub const COMPILER_JAR: &[u8] = include_bytes!("sql2dbsp-jar-with-dependencies.jar");
