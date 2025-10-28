use tonic_build::configure;

fn main() {
    const PROTOC_ENVAR: &str = "PROTOC";
    if std::env::var(PROTOC_ENVAR).is_err() {
        #[cfg(not(windows))]
        {
            if let Ok(protoc_path) = which::which("protoc") {
                std::env::set_var(PROTOC_ENVAR, protoc_path);
            } else {
                panic!("protoc not found. Install newer version.");
            }
        }
    }

    configure()
        .compile_protos(
            &[
                "protos/auth.proto",
                "protos/shared.proto",
                "protos/shredstream.proto",
            ],
            &["protos"],
        )
        .unwrap();
}
