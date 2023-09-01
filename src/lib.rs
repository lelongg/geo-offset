//! This crate contains algorithms to shrink and dilate various geometric shapes.
//!
//! This code is a Rust port of the JS lib [polygon-offset](https://github.com/w8r/polygon-offset).
//!
//! # Example
//!
//! The following example shows how to compute an inflated line.  
//! The [`offset`] method is provided by the [`Offset`] trait which is implemented for most [geo-types](https://docs.rs/geo-types/0.4.3/geo_types/).
//!
//! ```
//! # fn main() -> Result<(), geo_offset::OffsetError> {
//! use geo_types::{Coord, Line};
//! use geo_offset::Offset;
//!
//! let line = Line::new(
//!     Coord { x: 0.0, y: 0.0 },
//!     Coord { x: 1.0, y: 8.0 },
//! );
//!
//! let line_with_offset = line.offset(2.0)?;
//! # Ok(())
//! # }
//! ```
//!
//! [`Offset`]: offset/trait.Offset.html
//! [`offset`]: offset/trait.Offset.html#method.offset

mod edge;
pub use edge::*;

mod offset;
pub use offset::*;

#[cfg(test)]
mod tests;
