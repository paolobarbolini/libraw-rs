use libraw_sys as sys;

pub fn camera_list() -> Vec<String> {
    let mut list = Vec::new();
    let count = unsafe { sys::libraw_cameraCount() };
    let names = unsafe { sys::libraw_cameraList() };
    for i in 0..count {
        let name = unsafe { names.offset(i as isize) };
        let name = unsafe { std::ffi::CStr::from_ptr(*name) };
        let name = name.to_string_lossy().into_owned();
        list.push(name);
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_cameras() {
        let cameras = camera_list();
        assert!(!cameras.is_empty());
        // Check for some known cameras
        assert!(cameras.contains(&"Adobe Digital Negative (DNG)".to_string()));
        assert!(cameras.contains(&"Canon EOS R3".to_string()));
        assert!(cameras.contains(&"Leica M11".to_string()));
        assert!(cameras.contains(&"Nikon Z fc".to_string()));
        assert!(cameras.contains(&"Sony ILCE-7M4 (A7 IV)".to_string()));
    }
}
