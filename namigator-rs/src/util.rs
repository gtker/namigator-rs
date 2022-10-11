use crate::error::NamigatorError;
use std::ffi::CString;
use std::path::Path;

pub fn path_to_cstr(p: &Path) -> Result<CString, NamigatorError> {
    let e = match p.to_str() {
        None => return Err(NamigatorError::PathCStringConversion),
        Some(e) => e,
    };

    Ok(CString::new(e)?)
}
