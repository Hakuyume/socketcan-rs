use socketcan_alt::{Cmsg, Socket, Timestamping};
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
    socket.set_timestamping(Timestamping::SOFTWARE)?;
    socket.set_fd_frames(true)?;

    let mut cmsg_buf = [0; 64];
    loop {
        let (frame, cmsgs) = socket.recv_msg(&mut cmsg_buf)?;
        let timestamp = cmsgs.into_iter().flatten().find_map(|cmsg| match cmsg {
            Cmsg::Timestamping(ts) => Some(ts[0]),
            _ => None,
        });
        if let Some(timestamp) = timestamp {
            println!(
                "{:.9} {:?}",
                timestamp.tv_sec as f64 + timestamp.tv_nsec as f64 / 1_000_000_000.,
                frame
            );
        } else {
            println!("N/A {:?}", frame);
        }
    }
}
