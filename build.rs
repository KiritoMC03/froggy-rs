fn main() {
    println!("{}./vosk", rustc_link_search());
}

fn rustc_link_search<'a>() -> &'a str {
    "cargo:rustc-link-search="
}
