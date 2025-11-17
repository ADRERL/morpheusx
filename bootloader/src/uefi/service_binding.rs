//! UEFI Service Binding Protocol (UEFI Spec 2.10 Section 2.5.9)

pub type Handle = *mut core::ffi::c_void;
pub type Status = usize;

#[repr(C)]
pub struct ServiceBindingProtocol {
    pub create_child: unsafe extern "efiapi" fn(
        this: *mut ServiceBindingProtocol,
        child_handle: *mut Handle,
    ) -> Status,

    pub destroy_child: unsafe extern "efiapi" fn(
        this: *mut ServiceBindingProtocol,
        child_handle: Handle,
    ) -> Status,
}

