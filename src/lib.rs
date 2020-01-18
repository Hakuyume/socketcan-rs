//! Rust binding for [SocketCAN](https://www.kernel.org/doc/Documentation/networking/can.txt)
//!
//! ## Example
//!
//! ```no_run
//! use socketcan::{Frame, Socket, StandardFrame};
//! use std::ffi::CString;
//!
//! let socket = Socket::bind(CString::new("vcan0")?)?;
//! socket.set_recv_own_msgs(true)?;
//!
//! let frame = StandardFrame::new(0x42, &[0, 1, 2, 3, 4, 5, 6, 7]);
//! socket.send(&Frame::Standard(frame))?;
//!
//! let frame = socket.recv()?;
//! println!("{:?}", frame);
//!
//! # std::io::Result::Ok(())
//! ```

mod frame;
mod socket;
mod sys;

pub use frame::*;
pub use socket::Socket;
