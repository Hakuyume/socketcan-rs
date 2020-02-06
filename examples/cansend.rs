use socketcan_alt::*;
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

    let socket = Socket::bind(CString::new(opt.ifname)?)?;

    let id = if opt.id < 1 << 11 {
        Id::Standard(opt.id)
    } else {
        Id::Extended(opt.id)
    };
    let frame = match opt.brs {
        None => Frame::Data(DataFrame::new(id, &opt.data)),
        Some(brs) => {
            socket.set_fd_frames(true)?;
            Frame::FdData(FdDataFrame::new(id, brs, false, &opt.data))
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
