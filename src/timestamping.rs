use crate::sys;
use std::os::raw::c_int;

bitflags::bitflags! {
    pub struct Timestamping: c_int {
        const TX_HARDWARE = sys::SOF_TIMESTAMPING_TX_HARDWARE as _;
        const TX_SOFTWARE = sys::SOF_TIMESTAMPING_TX_SOFTWARE as _;
        const RX_HARDWARE = sys::SOF_TIMESTAMPING_RX_HARDWARE as _;
        const RX_SOFTWARE = sys::SOF_TIMESTAMPING_RX_SOFTWARE as _;
        const SOFTWARE = sys::SOF_TIMESTAMPING_SOFTWARE as _;
        const SYS_HARDWARE = sys::SOF_TIMESTAMPING_SYS_HARDWARE as _;
        const RAW_HARDWARE = sys::SOF_TIMESTAMPING_RAW_HARDWARE as _;
        const OPT_ID = sys::SOF_TIMESTAMPING_OPT_ID as _;
        const TX_SCHED = sys::SOF_TIMESTAMPING_TX_SCHED as _;
        const TX_ACK = sys::SOF_TIMESTAMPING_TX_ACK as _;
        const OPT_CMSG = sys::SOF_TIMESTAMPING_OPT_CMSG as _;
        const OPT_TSONLY = sys::SOF_TIMESTAMPING_OPT_TSONLY as _;
        const OPT_STATS = sys::SOF_TIMESTAMPING_OPT_STATS as _;
        const OPT_PKTINFO = sys::SOF_TIMESTAMPING_OPT_PKTINFO as _;
        const OPT_TX_SWHW = sys::SOF_TIMESTAMPING_OPT_TX_SWHW as _;
    }
}
