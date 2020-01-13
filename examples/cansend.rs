use nom::bytes::complete::{tag, take_while, take_while_m_n};
use nom::combinator::map_res;
use nom::multi::{many0, many_m_n};
use nom::IResult;
use socketcan::{CanFdFrame, CanFrame, CanSocket, Frame};
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

    let (input, frame) = parse_frame(&opt.frame).unwrap();
    assert!(input.is_empty());

    let socket = CanSocket::bind(CString::new(opt.ifname)?)?;
    socket.set_fd_frames(true)?;
    socket.send(&frame)?;

    Ok(())
}

fn is_digit(c: char) -> bool {
    c.is_digit(16)
}

fn parse_data(input: &str) -> IResult<&str, Vec<u8>> {
    many0(map_res(take_while_m_n(2, 2, is_digit), |s| {
        u8::from_str_radix(s, 16)
    }))(input)
}

fn parse_frame(input: &str) -> IResult<&str, Frame> {
    let (input, can_id) = map_res(take_while(is_digit), |s| u32::from_str_radix(s, 16))(input)?;
    let (input, sep) = many_m_n(1, 2, tag("#"))(input)?;
    match sep.len() {
        1 => {
            let (input, data) = parse_data(input)?;
            let frame = CanFrame::new(can_id, &data).unwrap();
            Ok((input, Frame::Can(frame)))
        }
        2 => {
            let (input, flags) = map_res(take_while_m_n(1, 1, is_digit), |s| {
                u8::from_str_radix(s, 16)
            })(input)?;
            let (input, data) = parse_data(input)?;
            let frame = CanFdFrame::new(can_id, flags, &data).unwrap();
            Ok((input, Frame::CanFd(frame)))
        }
        _ => unreachable!(),
    }
}
