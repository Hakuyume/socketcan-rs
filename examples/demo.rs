use futures::future::try_join;
use socketcan::async_await::{CanSocket, RecvHalf, SendHalf};
use socketcan::CanFdFrame;
use std::ffi::CString;
use std::io::Result;
use structopt::StructOpt;
use tokio::time::{delay_for, Duration};

#[derive(StructOpt)]
struct Opt {
    ifname: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let socket = CanSocket::bind(CString::new(opt.ifname)?)?;
    socket.set_recv_own_msgs(true)?;
    socket.set_fd_frames(true)?;

    let (rx, tx) = socket.split();
    try_join(recv(rx), send(tx)).await?;
    Ok(())
}

async fn recv(mut socket: RecvHalf) -> Result<()> {
    loop {
        let frame = socket.recv().await?;
        let data = frame
            .data()
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<String>();
        println!("{:03X}##{:X}{}", frame.can_id, frame.flags, data);
    }
}

async fn send(mut socket: SendHalf) -> Result<()> {
    let mut count = 0_u64;
    loop {
        let frame = CanFdFrame::new(42, 0, &count.to_be_bytes()).unwrap();
        socket.send(&frame).await?;
        count += 1;
        delay_for(Duration::new(1, 0)).await;
    }
}
