use socketcan::*;
use std::ffi::CString;
use std::io;
use std::num::ParseIntError;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    ifname: String,
    #[structopt(long, parse(try_from_str = parse_flags))]
    flags: Option<u8>,
    #[structopt(parse(try_from_str = parse_id))]
    id: u32,
    #[structopt(parse(try_from_str = parse_data))]
    data: std::vec::Vec<u8>,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let socket = CanSocket::bind(CString::new(opt.ifname)?)?;

    let is_extended = opt.id >= 2 << 11;
    let frame = match opt.flags {
        Some(flags) => {
            socket.set_fd_frames(true)?;
            if is_extended {
                CanFrame::FdExtended(CanFdExtendedFrame::new(opt.id, flags, &opt.data))
            } else {
                CanFrame::FdStandard(CanFdStandardFrame::new(opt.id, flags, &opt.data))
            }
        }
        None => {
            if is_extended {
                CanFrame::Extended(CanExtendedFrame::new(opt.id, &opt.data))
            } else {
                CanFrame::Standard(CanStandardFrame::new(opt.id, &opt.data))
            }
        }
    };
    socket.send(&frame)?;

    Ok(())
}

fn parse_id(src: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(src, 16)
}

fn parse_flags(src: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(src, 16)
}

fn parse_data(src: &str) -> Result<Vec<u8>, ParseIntError> {
    src.chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|src| u8::from_str_radix(&src.iter().collect::<String>(), 16))
        .collect()
}
