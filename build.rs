use std::{env::var, ffi::OsString, fmt::Write, fs};

fn main() {
    println!("cargo:rerun-if-changed=.");
    let mut out_file = String::new();
    for file in fs::read_dir(var("CARGO_MANIFEST_DIR").unwrap() + "/src/solutions").unwrap() {
        let path = file.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if path.extension() == Some(&OsString::from("rs")) && file_name != "build.rs" {
            println!("{file_name}");
            write!(
                &mut out_file,
                "#[path = \"../../../../../src/solutions/{}\"] mod {};",
                file_name,
                file_name.split('.').nth(1).unwrap().replace('-', "_")
            )
            .unwrap();
        }
    }

    let out_dir = var("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir);
    fs::write(out_dir.join("problems.rs"), out_file).unwrap();
}
