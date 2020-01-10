use socketcan::CanSocket;
use std::io::Result;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    ifname: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let socket = CanSocket::new()?;
    socket.set_fd_frames(true)?;
    socket.bind(opt.ifname)?;

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
