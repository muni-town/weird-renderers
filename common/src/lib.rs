extern crate alloc;
use core::alloc::Layout;
use std::{panic::PanicHookInfo, sync::OnceLock};

use serde::{Deserialize, Serialize};
pub use serde_json;

#[derive(Serialize, Deserialize)]
pub struct ProfileData {
    pub instance_info: InstanceInfo,
    #[serde(default)]
    pub handle: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub bio: Option<String>,
    #[serde(default)]
    pub social_links: Vec<SocialLinkInfo>,
    #[serde(default)]
    pub links: Vec<LinkInfo>,
    #[serde(default)]
    pub pages: Vec<PageInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct InstanceInfo {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct SocialLinkInfo {
    pub url: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub platform_name: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub icon_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LinkInfo {
    pub url: String,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PageInfo {
    pub slug: String,
    #[serde(default)]
    pub name: Option<String>,
}

pub mod console {
    mod wasm {

        #[link(wasm_import_module = "console")]
        extern "C" {
            pub fn error(ptr: *const u8, len: usize);
        }
    }

    pub fn error(msg: &str) {
        unsafe {
            wasm::error(msg.as_ptr(), msg.len());
        }
    }
}

fn panic_hook(info: &PanicHookInfo) {
    console::error(&format!("{info}"))
}

static ONCE: OnceLock<()> = OnceLock::new();
pub fn set_panic_hook_once() {
    ONCE.get_or_init(|| std::panic::set_hook(Box::new(panic_hook)));
}

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

#[no_mangle]
pub static mut OUTPUT: (*mut u8, usize) = (std::ptr::null_mut(), 0);

#[no_mangle]
pub static mut OUTPUT_PTR: *mut u8 = std::ptr::null_mut();
pub static mut DROP_OUTPUT: fn() = null_fn;
fn null_fn() {}

#[no_mangle]
unsafe extern "C" fn drop_output() {
    (DROP_OUTPUT)()
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
        ) {
            $crate::set_panic_hook_once();

            unsafe {
                let profile_data_json_bytes =
                    ::core::slice::from_raw_parts_mut(profile_data_json_ptr, profile_data_json_len);
                let profile_data: ProfileData =
                    $crate::serde_json::from_slice(profile_data_json_bytes).unwrap();
                let theme = ::core::slice::from_raw_parts_mut(theme_data, theme_data_len);
                let mut result: String = $function(profile_data, theme);

                let mut out = Box::new(result);
                OUTPUT = (out.as_mut_ptr(), out.len());
                OUTPUT_PTR = Box::into_raw(out) as *mut u8;
                DROP_OUTPUT = || {
                    std::ptr::drop_in_place(OUTPUT_PTR as *mut String);
                }
            }
        }
    };
}
