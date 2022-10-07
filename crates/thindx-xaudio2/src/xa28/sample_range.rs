#[allow(unused_imports)] use super::xaudio2::sys::*;
use bytemuck::Zeroable;
use winresult::*;
use core::convert::Infallible;
use core::ops::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer)\] [XAUDIO2_BUFFER]::{\*Begin, \*Length} tuple - as measured in "samples".
///
/// Somewhat similar to a storable [RangeBounds]<[u32]>.  Generally corresponds to one of:
/// *   [XAUDIO2_BUFFER]::{[PlayBegin](XAUDIO2_BUFFER::PlayBegin), [PlayLength](XAUDIO2_BUFFER::PlayLength)}
/// *   [XAUDIO2_BUFFER]::{[LoopBegin](XAUDIO2_BUFFER::LoopBegin), [LoopLength](XAUDIO2_BUFFER::LoopLength)}
///
/// The internal representation of this type doesn't 100% match [XAUDIO2_BUFFER]'s fields.  Notably:
/// *   `(PlayBegin, PlayLength) == (0, 0)` means "play the full buffer"
/// *   `(PlayBegin, PlayLength) == (x, 0)` is generally invalid (but this type uses it to mean "empty")
/// *   `(LoopBegin, LoopLength, LoopCount) == (0, 0, 0)` means no loop region
/// *   `(LoopBegin, LoopLength, LoopCount) == (0, 0, n)` means "loop the full buffer" some number of times
/// *   `(LoopBegin, LoopLength, LoopCount) == (x, y, 0)` is generally invalid?
/// *   `(LoopBegin, LoopLength, LoopCount) == (x, 0, z)` is generally invalid? probably?
///
/// Whereas this internally uses:
/// *   `(begin, length) == (1, 0)` to indicate an empty region, which is handled differently for play and loop regions.
#[derive(Clone, Copy, Debug, Default, Zeroable, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SampleRange {
    // Internal impl doesn't 100% match XAUDIO2_BUFFER.
    // Namely, [SampleRange::EMPTY] exists.
    begin:  u32,
    length: u32,
}

impl SampleRange {
    pub const FULL  : SampleRange = SampleRange { begin: 0, length: 0 };
    pub const EMPTY : SampleRange = SampleRange { begin: 1, length: 0 };
    pub const fn is_empty(&self) -> bool { self.length == 0 && self.begin != 0 }

    /// | self          | Return |
    /// | ------------- | ------ |
    /// | [Self::EMPTY] | None                      |
    /// | [Self::FULL]  | Some((0, 0)) ⚠️           |
    /// | start .. end  | Some((start, end-start))  |
    pub(crate) const fn into_raw_xaudio2_begin_length(&self) -> Option<(u32, u32)> { if self.is_empty() { None } else { Some((self.begin, self.length)) } }
}

impl From<Option<Infallible>> for SampleRange { fn from(_: Option<Infallible>) -> Self { Self::EMPTY } }
impl From<RangeFull> for SampleRange { fn from(_: RangeFull) -> Self { Self::FULL } }

impl TryFrom<RangeTo<u32>> for SampleRange {
    type Error = HResultError;
    fn try_from(value: RangeTo<u32>) -> Result<Self, Self::Error> { Self::try_from(0 .. value.end) }
}

impl TryFrom<RangeToInclusive<u32>> for SampleRange {
    type Error = HResultError;
    fn try_from(value: RangeToInclusive<u32>) -> Result<Self, Self::Error> { Self::try_from(0 .. value.end.checked_add(1).ok_or(E::INVALIDARG)?) }
}

impl TryFrom<RangeInclusive<u32>> for SampleRange {
    type Error = HResultError;
    fn try_from(value: RangeInclusive<u32>) -> Result<Self, Self::Error> { Self::try_from(*value.start() .. value.end().checked_add(1).ok_or(E::INVALIDARG)?) }
}

impl TryFrom<Range<u32>> for SampleRange {
    type Error = HResultError;
    fn try_from(value: Range<u32>) -> Result<Self, Self::Error> {
        let begin  = value.start;
        let length = value.end.checked_sub(begin).ok_or(E::INVALIDARG)?;
        if length == 0 { return Ok(Self::EMPTY) } // don't accidentally create Self::FULL
        Ok(Self { begin, length })
    }
}
