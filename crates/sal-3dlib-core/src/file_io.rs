use sal_core::dbg::Dbg;
use sal_core::error::Error;
use std::path::PathBuf;
///
pub fn remove(_: &Dbg, path: &PathBuf)  {
    let _ = std::fs::remove_file(path);
}
///
pub fn create_dir(dbg: &Dbg, dir_path: &PathBuf) -> Result<(), Error> {
    let error = Error::new(dbg, "save");    
    std::fs::create_dir_all(dir_path).map_err(|err| {
        error.pass_with(
            format!("std::fs::create_dir_all error! path:{}", dir_path.display()),
            err.to_string(),
        )
    })?;
    Ok(())
}
