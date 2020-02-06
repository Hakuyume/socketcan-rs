use socketcan_alt::Socket;
use std::ffi::CString;
use std::io::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    ifname: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let socket = Socket::bind(CString::new(opt.ifname)?)?;
    socket.set_fd_frames(true)?;

    loop {
        println!("{:?}", socket.recv()?)
    }
}
