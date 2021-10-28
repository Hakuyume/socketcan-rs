#![allow(clippy::unreadable_literal)]
#![allow(clippy::unseparated_literal_suffix)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "can-dlc-unaliased")]
impl can_frame {
    pub(crate) fn len(&self) -> u8 {
        self.can_dlc
    }

    pub(crate) unsafe fn set_len(&mut self, len: u8) {
        self.can_dlc = len;
    }
}

#[cfg(not(feature = "can-dlc-unaliased"))]
impl can_frame {
    pub(crate) fn len(&self) -> u8 {
        unsafe { self.__bindgen_anon_1.len }
    }

    pub(crate) unsafe fn set_len(&mut self, len: u8) {
        self.__bindgen_anon_1.len = len;
    }
}
