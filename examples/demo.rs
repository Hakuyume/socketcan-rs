use socketcan::*;
use std::ffi::CString;
use std::io::Result;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    ifname: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let socket = Arc::new(CanSocket::bind(CString::new(opt.ifname)?)?);
    socket.set_recv_own_msgs(true)?;
    socket.set_fd_frames(true)?;

    {
        let socket = socket.clone();
        thread::spawn(move || -> Result<()> {
            loop {
                println!("{:?}", socket.recv()?)
            }
        });
    }

    let mut count = 0_u64;
    loop {
        let frame = if count % 15 == 0 {
            CanFrame::FdExtended(CanFdExtendedFrame::new(42, 0, &count.to_be_bytes()))
        } else if count % 3 == 0 {
            CanFrame::Extended(CanExtendedFrame::new(42, &count.to_be_bytes()))
        } else if count % 5 == 0 {
            CanFrame::FdStandard(CanFdStandardFrame::new(42, 0, &count.to_be_bytes()))
        } else {
            CanFrame::Standard(CanStandardFrame::new(42, &count.to_be_bytes()))
        };
        socket.send(&frame)?;
        count += 1;
        thread::sleep(Duration::new(1, 0));
    }
}
