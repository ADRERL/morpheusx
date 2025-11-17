// Installer menu state
use crate::installer::EspInfo;
use alloc::vec::Vec;

pub struct InstallerMenu {
    pub esp_list: Vec<EspInfo>,
    pub selected_esp: usize,
    pub scan_complete: bool,
    pub image_handle: *mut (),
}

impl InstallerMenu {
    pub fn new(image_handle: *mut ()) -> Self {
        Self {
            esp_list: Vec::new(),
            selected_esp: 0,
            scan_complete: false,
            image_handle,
        }
    }
}
