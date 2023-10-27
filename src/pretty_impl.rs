/// Strips std path-noise from [`type_name()`](std::any::type_name).
///
/// You can customize the behavior by using a [cargo patch].
///
/// [cargo patch]: https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html?#the-patch-section
pub fn pretty(name: &str) -> &str {
    // FIXME(rust): This can be const once type_name is.
    let pretty = include!("pretty.expr.rs");
    for &(bad, good) in pretty {
        if name.starts_with(bad) {
            let name = &name[bad.len()..];
            if name.starts_with(good) {
                return name;
            }
        }
    }
    name
}
