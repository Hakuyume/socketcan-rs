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
        let frame = socket.recv()?;
        let data = frame
            .data()
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<String>();
        println!("{:03X}##{:X}{}", frame.can_id, frame.flags, data);
    }
}
