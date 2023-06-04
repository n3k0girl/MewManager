
pub fn main(pkg_name: Option<String>) {
    let pkg: String = checkFormat(pkg_name);
    println!("{}", pkg);
}

fn checkFormat(pkg_name: Option<String>) -> String {
    if pkg_name.is_some() {
        return pkg_name.expect("We can't convert your package name.").to_string();
    } else {
        "NoPKG".to_string()
    }
}