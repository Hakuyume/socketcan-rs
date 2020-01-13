use nom::{
    bytes::complete::{tag, take_while, take_while_m_n},
    combinator::map_res,
    multi::many0,
    IResult,
};
use socketcan::{CanFdFrame, CanSocket};
use std::error::Error;
use std::ffi::CString;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    ifname: String,
    frame: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let (input, (can_id, flags, data)) = parse_frame(&opt.frame).unwrap();
    assert!(input.is_empty());
    let frame = CanFdFrame::new(can_id, flags, &data).unwrap();

    let socket = CanSocket::bind(CString::new(opt.ifname)?)?;
    socket.set_fd_frames(true)?;
    socket.send(&frame)?;

    Ok(())
}

fn is_digit(c: char) -> bool {
    c.is_digit(16)
}

fn parse_frame(input: &str) -> IResult<&str, (u32, u8, Vec<u8>)> {
    let (input, can_id) = map_res(take_while(is_digit), |s| u32::from_str_radix(s, 16))(input)?;
    let (input, _) = tag("##")(input)?;
    let (input, flags) = map_res(take_while_m_n(1, 1, is_digit), |s| {
        u8::from_str_radix(s, 16)
    })(input)?;
    let (input, data) = many0(map_res(take_while_m_n(2, 2, is_digit), |s| {
        u8::from_str_radix(s, 16)
    }))(input)?;
    Ok((input, (can_id, flags, data)))
}
