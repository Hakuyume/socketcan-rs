use socketcan::{CanFdFrame, CanSocket};
use std::io::Result;

fn main() -> Result<()> {
    let socket = CanSocket::bind("vcan0")?;
    socket.set_can_fd_frames(true)?;

    let frame = CanFdFrame::new(42, &[0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x23, 0x45, 0x67])?;
    socket.write_frame(&frame)?;

    Ok(())
}
