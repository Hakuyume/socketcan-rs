use socketcan::{CanSocket, Frame};
use std::ffi::CString;
use std::io::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    ifname: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let socket = CanSocket::bind(CString::new(opt.ifname)?)?;
    socket.set_fd_frames(true)?;

    loop {
        match socket.recv()? {
            Frame::Can(frame) => println!("{:03X}#{}", frame.can_id(), hex(frame.data())),
            Frame::CanFd(frame) => println!(
                "{:03X}##{:X}{}",
                frame.can_id(),
                frame.flags(),
                hex(frame.data())
            ),
        }
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.into_iter().map(|b| format!("{:02X}", b)).collect()
}
