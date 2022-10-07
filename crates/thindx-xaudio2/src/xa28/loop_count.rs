use super::xaudio2::sys::*;

use bytemuck::*;
use nonmax::NonMaxU8;
use winresult::*;

use core::convert::Infallible;
use core::fmt::{self, Debug, Display, Formatter};
use core::ops::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer)\]
/// `0 ..= 254` \| `∞`
///
/// Differences from [XAUDIO2_BUFFER::LoopCount]:
/// *   Internally [u8], not [u32] ([XAUDIO2_MAX_LOOP_COUNT] is 254, [XAUDIO2_LOOP_INFINITE] is 255, and above is invalid.)
/// *   Attempts to prevent accidental construction of a 255 ("infinite") loop count.
///     *   [new](Self::new) requires [NonMaxU8].
///     *   [TryFrom]<[u8]> errors on 255.
#[derive(Clone, Copy, Default, Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoopCount(pub(crate) u8);

/// No looping.
pub const NO_LOOP_REGION    : LoopCount = LoopCount(  0);
/// Max finite loop count for a single submit (254).
pub const MAX_LOOP_COUNT    : LoopCount = LoopCount(254);
/// Loop infinitely / forever.
pub const LOOP_INFINITE     : LoopCount = LoopCount(255);

const _ : () = assert!((NO_LOOP_REGION.0 as u32) == XAUDIO2_NO_LOOP_REGION);
const _ : () = assert!((MAX_LOOP_COUNT.0 as u32) == XAUDIO2_MAX_LOOP_COUNT);
const _ : () = assert!((LOOP_INFINITE .0 as u32) == XAUDIO2_LOOP_INFINITE );

const _ : () = assert!((LoopCount::NO_LOOP .0 as u32) == XAUDIO2_NO_LOOP_REGION);
const _ : () = assert!((LoopCount::MAX     .0 as u32) == XAUDIO2_MAX_LOOP_COUNT);
const _ : () = assert!((LoopCount::INFINITE.0 as u32) == XAUDIO2_LOOP_INFINITE );

impl LoopCount {
    /// No looping.
    pub const NO_LOOP   : Self = Self(0);
    /// Max finite loop count for a single submit (254).
    pub const MAX       : Self = Self(254);
    /// Loop infinitely / forever.
    pub const INFINITE  : Self = Self(255);

    pub const fn new(value: NonMaxU8) -> Self { Self(value.get()) }
    pub const fn is_finite(  self) -> bool { self.0 != LOOP_INFINITE.0 }
    pub const fn is_infinite(self) -> bool { self.0 == LOOP_INFINITE.0 }

    /// [Some]\(loop_count\) if [is_finite](Self::is_finite), [None] if [is_infinite](Self::is_infinite).
    pub const fn finite(self) -> Option<NonMaxU8> { NonMaxU8::new(self.0) }
}

impl Debug   for LoopCount { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { if self.is_infinite() { write!(fmt, "∞") } else { write!(fmt, "{}", self.0) } } }
impl Display for LoopCount { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { if self.is_infinite() { write!(fmt, "∞") } else { write!(fmt, "{}", self.0) } } }

impl From<NonMaxU8>             for LoopCount { fn from(v: NonMaxU8             ) -> Self { Self(v.get())   } }
impl From<Option<Infallible>>   for LoopCount { fn from(_: Option<Infallible>   ) -> Self { NO_LOOP_REGION  } }
impl From<RangeFull>            for LoopCount { fn from(_: RangeFull            ) -> Self { LOOP_INFINITE   } }

impl TryFrom<u8> for LoopCount {
    type Error = HResultError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 255 { Err(E::INVALIDARG) }
        else            { Ok(Self(value)) }
    }
}
