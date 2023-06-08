pub fn checkFormat(pkg_name: Option<String>) -> String {
    if pkg_name.is_some() {
        return pkg_name.expect("We can't convert your package name.").to_string();
    } else {
        return "NoPKG".to_string();
    }
}