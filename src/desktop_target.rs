#![allow(dead_code, non_snake_case)]

use ::windows::{Abi, Guid, IUnknown, Interface, RawPtr, Result, HRESULT};
use bindings::Windows::Win32::UI::WindowsAndMessaging::HWND;

#[repr(transparent)]
pub struct IDesktopWindowXamlSourceNative(IUnknown);

impl IDesktopWindowXamlSourceNative {
    pub fn AttachToWindow(&self, hwnd: HWND) -> Result<()> {
        unsafe { (self.vtable().3)(self.abi(), hwnd.0 as RawPtr).ok() }
    }

    pub fn get_WindowHandle(&self) -> Result<HWND> {
        unsafe {
            let mut hwnd = std::ptr::null_mut();
            (self.vtable().4)(self.abi(), &mut hwnd).and_then(|| HWND(hwnd as isize))
        }
    }
}

#[repr(C)]
pub struct IDesktopWindowXamlSourceNative_vtable(
    pub unsafe extern "system" fn(this: RawPtr, iid: &Guid, interface: *mut RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr) -> u32,
    pub unsafe extern "system" fn(this: RawPtr, hwnd: RawPtr) -> HRESULT,
    pub unsafe extern "system" fn(this: RawPtr, hwnd_out: *mut RawPtr) -> HRESULT,
);

unsafe impl Interface for IDesktopWindowXamlSourceNative {
    type Vtable = IDesktopWindowXamlSourceNative_vtable;

    const IID: Guid = Guid::from_values(
        0x3CBC_F1BF,
        0x2F76,
        0x4E9C,
        [0x96, 0xAB, 0xE8, 0x4B, 0x37, 0x97, 0x25, 0x54],
    );
}
