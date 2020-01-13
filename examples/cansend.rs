use socketcan::{CanFdFrame, CanFrame, CanSocket, Frame};
use std::ffi::CString;
use std::io;
use std::num::ParseIntError;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    ifname: String,
    #[structopt(long, parse(try_from_str = parse_flags))]
    flags: Option<u8>,
    #[structopt(parse(try_from_str = parse_can_id))]
    can_id: u32,
    #[structopt(parse(try_from_str = parse_data))]
    data: std::vec::Vec<u8>,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let socket = CanSocket::bind(CString::new(opt.ifname)?)?;
    let frame = match opt.flags {
        Some(flags) => {
            socket.set_fd_frames(true)?;
            Frame::CanFd(CanFdFrame::new(opt.can_id, flags, &opt.data))
        }
        None => Frame::Can(CanFrame::new(opt.can_id, &opt.data)),
    };
    socket.send(&frame)?;

    Ok(())
}

fn parse_can_id(src: &str) -> Result<u32, ParseIntError> {
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
