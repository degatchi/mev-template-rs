use ethers_contract::MultiAbigen;

fn main() {
    let input_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/abi");
    let output_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings");

    let gen = MultiAbigen::from_json_files(input_path).unwrap();
    let bindings = gen.build().unwrap();
    bindings.write_to_module(output_path, false).unwrap();

    println!("cargo:rerun-if-changed={}", input_path)
}