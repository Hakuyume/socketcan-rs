bitflags::bitflags! {
    pub struct Timestamping: u32 {
        const TX_HARDWARE = libc::SOF_TIMESTAMPING_TX_HARDWARE;
        const TX_SOFTWARE = libc::SOF_TIMESTAMPING_TX_SOFTWARE;
        const RX_HARDWARE = libc::SOF_TIMESTAMPING_RX_HARDWARE;
        const RX_SOFTWARE = libc::SOF_TIMESTAMPING_RX_SOFTWARE;
        const SOFTWARE = libc::SOF_TIMESTAMPING_SOFTWARE;
        const SYS_HARDWARE = libc::SOF_TIMESTAMPING_SYS_HARDWARE;
        const RAW_HARDWARE = libc::SOF_TIMESTAMPING_RAW_HARDWARE;
    }
}
