//! Compile-time PE constants extracted from linker at build time

pub const LINKER_IMAGE_BASE_STR: Option<&str> = option_env!("MORPHEUS_IMAGE_BASE");

pub fn get_original_image_base_hint() -> Option<u64> {
    LINKER_IMAGE_BASE_STR.and_then(|s| {
        let s = s.trim_start_matches("0x").trim_start_matches("0X");
        u64::from_str_radix(s, 16).ok()
    })
}

