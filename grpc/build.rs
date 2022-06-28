use std::path::Path;
use std::error::Error;

const PROTO_DIR: &str = "./proto";
const PROTO_EXT: &str = ".proto";

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(PROTO_DIR);
    for entry in path.read_dir()? {
        let path = entry?.path();
        if path.to_string_lossy().ends_with(PROTO_EXT) {
            println!("{:?}", path);
            tonic_build::compile_protos(path)?;
        }
    }
    Ok(())
}
