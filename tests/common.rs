#![allow(unused_imports)]
#![allow(dead_code)]

pub use std::path::Path;
use std::path::PathBuf;

pub fn bin_path() -> &'static Path {
    const BIN_PATH: &str = env!("CARGO_BIN_EXE_upfind");
    Path::new(BIN_PATH)
}

pub fn tmp_root() -> &'static Path {
    const TMP_PATH: &str = env!("CARGO_TARGET_TMPDIR");
    Path::new(TMP_PATH)
}

pub fn tmp_for<P: AsRef<Path>>(id: P) -> std::io::Result<PathBuf> {
    let p = tmp_root().join(id);

    // create directory if it does not exist
    if !p.exists() {
        std::fs::create_dir_all(&p)?;
    }

    assert!(p.exists());

    Ok(p)
}
