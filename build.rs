use camino::Utf8Path;

fn main() {
    let udl_file = "./src/api.udl";
    let out_dir = "./bindings"; 

    std::fs::create_dir_all(out_dir).unwrap();

    println!("cargo:rerun-if-changed={}", udl_file);

    uniffi::generate_scaffolding(udl_file).unwrap();
    
    let udl_path = Utf8Path::new(udl_file);
    uniffi_dart::generate_scaffolding(udl_path).unwrap();
}
