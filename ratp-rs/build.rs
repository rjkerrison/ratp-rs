use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let _ = savon::gen::gen_write("./assets/ratp-wsiv-opendata/Wsiv.wsdl", &out_dir);
}
