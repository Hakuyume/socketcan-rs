//! Rust binding for [SocketCAN](https://www.kernel.org/doc/Documentation/networking/can.txt)
//!
//! ## Example
//!
//! ```no_run
//! use socketcan::{CanFrame, CanSocket, CanStandardFrame};
//! use std::ffi::CString;
//!
//! let socket = CanSocket::bind(CString::new("vcan0")?)?;
//! socket.set_recv_own_msgs(true)?;
//!
//! let frame = CanStandardFrame::new(0x42, &[0, 1, 2, 3, 4, 5, 6, 7]);
//! socket.send(&CanFrame::Standard(frame))?;
//!
//! let frame = socket.recv()?;
//! println!("{:?}", frame);
//!
//! # std::io::Result::Ok(())
//! ```

mod can_frame;
mod can_socket;
mod sys;

pub use can_frame::*;
pub use can_socket::CanSocket;
