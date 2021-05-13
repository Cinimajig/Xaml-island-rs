#![allow(dead_code)]

use bindings::Windows::Win32::System::WinRT::{RoInitialize, RoUninitialize, RO_INIT_TYPE};

pub struct RoInit;

impl RoInit {
    pub fn multi_threaded() -> windows::Result<Self> {
        unsafe {
            match RoInitialize(RO_INIT_TYPE(0)).ok() {
                Ok(_) => Ok(Self),
                Err(err) => Err(err),
            }
        }
    }
}

impl Drop for RoInit {
    fn drop(&mut self) {
        unsafe {
            RoUninitialize();
        }
    }
}
