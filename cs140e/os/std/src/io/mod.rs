#[cfg(feature = "nightly")]
mod buffered;
mod cursor;
mod error;
mod impls;
mod traits;
mod util;

#[cfg(not(feature = "std"))]
pub use self::cursor::Cursor;
#[cfg(not(feature = "std"))]
pub use self::error::{Error, ErrorKind, Result};
#[cfg(not(feature = "std"))]
pub use self::traits::{BufRead, Bytes, Chain, Read, Seek, SeekFrom, Take, Write};

#[cfg(feature = "std")]
pub use std::io::{
    BufRead, Bytes, Chain, Cursor, Error, ErrorKind, Read, Result, Seek, SeekFrom, Take, Write,
};

// Use this crate's implementation on both std and no_std
#[cfg(feature = "nightly")]
pub use self::buffered::{BufReader, BufWriter, LineWriter};

#[cfg(feature = "nightly")]
pub use self::util::copy;
