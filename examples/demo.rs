use socketcan::{AsyncCanSocket, CanFdFrame};
use std::io::Result;
use structopt::StructOpt;
use tokio::time::{Duration, delay_for};
use futures::future::try_join;

#[derive(StructOpt)]
struct Opt {
    ifname: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let socket = AsyncCanSocket::new()?;
    socket.set_recv_own_msgs(true)?;
    socket.set_fd_frames(true)?;
    socket.bind(opt.ifname)?;

    try_join(recv(&socket), send(&socket)).await?;
    Ok(())
}

async fn recv(socket: &AsyncCanSocket) -> Result<()> {
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

async fn send(socket: &AsyncCanSocket) -> Result<()> {
    let mut count = 0_u64;
    loop {
        let frame = CanFdFrame::new(42, 0, &count.to_be_bytes()).unwrap();
        socket.send(&frame).await?;
        count+= 1;
        delay_for(Duration::new(1, 0)).await;
    }
}
