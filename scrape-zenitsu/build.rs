extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["src/protos/input.proto", "src/protos/output.proto"],
        &["src/"],
    )
    .unwrap();
}
