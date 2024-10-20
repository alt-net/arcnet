include!(concat!(env!("CARGO_MANIFEST_DIR"), "/_uuid.rs"));

#[allow(dead_code)]
pub const fn uuid() -> &'static str {
    UUID
}