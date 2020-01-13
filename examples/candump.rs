use socketcan::CanSocket;
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
        println!("{:?}", socket.recv()?)
    }
}
