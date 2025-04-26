use std::path::Path;

fn main() {
    let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!(
        "cargo::rustc-link-arg-bins=--script={}",
        cargo_dir.join("linker.ld").display()
    );
}
