fn main() {
    tonic_build::compile_protos("proto/MyNoSqlServerPersistence.proto").unwrap();
}
