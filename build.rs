fn main() {
    tonic_build::compile_protos("protocols/control/proxy.proto").unwrap();
}