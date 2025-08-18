use st_mems_reg_config_conv::parser;
use std::path::Path;

fn main() {
    let input_file = Path::new("norm.json");
    let output_file = Path::new("src/ispu_norm.rs");
    parser::generate_rs_from_json(&input_file, &output_file, "NORM_PROGRAM", "ISM330IS", true);

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
}
