use socketcan::*;
use std::ffi::CString;
use std::io;
use std::num::ParseIntError;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    ifname: String,
    #[structopt(long)]
    brs: Option<bool>,
    #[structopt(parse(try_from_str = parse_id))]
    id: u32,
    #[structopt(parse(try_from_str = parse_data))]
    data: std::vec::Vec<u8>,
}

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    let socket = CanSocket::bind(CString::new(opt.ifname)?)?;

    let frame = match opt.brs {
        None => {
            if opt.id < 1 << CanStandardFrame::ID_BITS {
                CanFrame::Standard(CanStandardFrame::new(opt.id, &opt.data))
            } else {
                CanFrame::Extended(CanExtendedFrame::new(opt.id, &opt.data))
            }
        }
        Some(brs) => {
            socket.set_fd_frames(true)?;
            if opt.id < 1 << CanFdStandardFrame::ID_BITS {
                CanFrame::FdStandard(CanFdStandardFrame::new(opt.id, brs, false, &opt.data))
            } else {
                CanFrame::FdExtended(CanFdExtendedFrame::new(opt.id, brs, false, &opt.data))
            }
        }
    };
    socket.send(&frame)?;

    Ok(())
}

fn parse_id(src: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(src, 16)
}

fn parse_data(src: &str) -> Result<Vec<u8>, ParseIntError> {
    src.chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|src| u8::from_str_radix(&src.iter().collect::<String>(), 16))
        .collect()
}
