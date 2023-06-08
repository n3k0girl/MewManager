use reqwest;

pub fn main(pkg_name: String) {
    println!("{}", pkg_name);
    checkOnline();
}

fn checkOnline() {
    let resp = match reqwest::blocking::get("https://raw.githubusercontent.com/n3k0girl/MewMirror/main/mirror.mew") {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err)
    };
   println!("{}", resp)
}