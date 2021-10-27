#![allow(clippy::unreadable_literal)]
#![allow(clippy::unseparated_literal_suffix)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub unsafe fn can_frame_len(frame: *const can_frame) -> u8 {
    #[cfg(feature = "can-dlc-unaliased")]
    {
        (*frame).can_dlc
    }

    #[cfg(not(feature = "can-dlc-unaliased"))]
    {
        (*frame).__bindgen_anon_1.len
    }
}

pub unsafe fn can_frame_len_mut(frame: *mut can_frame) -> *mut u8 {
    #[cfg(feature = "can-dlc-unaliased")]
    {
        &mut (*frame).can_dlc
    }

    #[cfg(not(feature = "can-dlc-unaliased"))]
    {
        &mut (*frame).__bindgen_anon_1.len
    }
}
