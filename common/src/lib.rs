extern crate alloc;
use core::alloc::Layout;

use serde::{Deserialize, Serialize};
pub use serde_json_core;

/// Expose a function so the host can allocate memory in this module
#[no_mangle]
extern "C" fn wasm_alloc(size: usize, align: usize) -> *mut u8 {
    unsafe { alloc::alloc::alloc(Layout::from_size_align(size, align).unwrap_unchecked()) }
}

/// Expose a function so the host can deallocate memory in this module
#[no_mangle]
extern "C" fn wasm_dealloc(ptr: *mut u8, size: usize, align: usize) {
    unsafe { alloc::alloc::dealloc(ptr, Layout::from_size_align_unchecked(size, align)) }
}

/// Use this macro to set the render function.
///
/// # Example
/// ```rust,ignore
/// fn my_render_function(profile: ProfileData, theme: Vec<u8>) -> String {
///     format!("Hi {}!", profile.name)
/// }
/// render_function!(my_render_function);
/// ```
#[macro_export]
macro_rules! render_function {
    ($function:ident) => {
        #[no_mangle]
        #[allow(improper_ctypes_definitions)] // Returning a tuple is valid for WASM multi-return
        extern "C" fn wasm_render(
            profile_data_json_ptr: *mut u8,
            profile_data_json_len: usize,
            theme_data: *mut u8,
            theme_data_len: usize,
        ) -> (*mut u8, usize) {
            unsafe {
                let profile_data_json_bytes =
                    ::core::slice::from_raw_parts_mut(profile_data_json_ptr, profile_data_json_len);
                let (profile_data, _): (ProfileData, _) =
                    $crate::serde_json_core::from_slice(profile_data_json_bytes).unwrap();
                let theme = ::core::slice::from_raw_parts_mut(theme_data, theme_data_len);
                let mut result: String = $function(profile_data, theme);

                let ptr = result.as_mut_ptr();
                let len = result.len();
                (ptr, len)
            }
        }
    };
}

#[derive(Serialize, Deserialize)]
pub struct ProfileData {
    pub name: String,
}
