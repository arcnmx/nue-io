#![deny(missing_docs)]

//! Utilities for working with I/O streams.

extern crate uninitialized;
extern crate resize_slice;

mod seek_forward;

mod buf_seeker;
mod region;
mod align;
mod take;

pub use seek_forward::{
    SeekRewind, SeekForward, SeekBackward, SeekAbsolute, SeekEnd, Tell,
    ReadWriteTell, SeekForwardRead, SeekForwardWrite, SeekAbsoluteRewind, SeekAll
};
pub use buf_seeker::BufSeeker;
pub use region::Region;
pub use align::SeekAlignExt;
pub use take::Take;
