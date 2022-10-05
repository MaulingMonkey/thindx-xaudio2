use super::xaudio2::sys::*;

use winresult::{HResultError, E};



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer)\] [XAUDIO2_BUFFER]
#[allow(non_snake_case)] // XXX: is this really what we want?
#[derive(Clone, Copy, Debug, Default)] #[repr(C)] pub struct Buffer<'a, Context: Send + Sync + Sized + 'static> {
    /// Either 0 or [XAUDIO2_END_OF_STREAM].
    pub Flags: u32,

    /// Pointer to the audio data buffer.
    pub AudioData: &'a [u8],

    /// First sample in this buffer to be played.
    pub PlayBegin: u32,

    /// Length of the region to be played in samples, or 0 to play the whole buffer.
    pub PlayLength: u32,

    /// First sample of the region to be looped.
    pub LoopBegin: u32,

    /// Length of the desired loop region in samples, or 0 to loop the entire buffer.
    pub LoopLength: u32,

    /// Number of times to repeat the loop region, or XAUDIO2_LOOP_INFINITE to loop forever.
    pub LoopCount: u32,

    /// Context value to be passed back in callbacks.
    pub Context: Context,
}

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer_wma)\] [XAUDIO2_BUFFER_WMA]
#[allow(non_snake_case)] // XXX: is this really what we want?
#[derive(Clone, Copy, Debug, Default)] pub struct BufferWma<'a> {
    /// Decoded packet cumulative data size array, each element is the number of bytes accumulated after the corresponding xWMA packet is decoded in order.
    pub DecodedPacketCumulativeBytes: &'a [u32],
}

impl<'a> TryFrom<BufferWma<'a>> for XAUDIO2_BUFFER_WMA {
    type Error = HResultError;
    fn try_from(buffer_wma: BufferWma<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            pDecodedPacketCumulativeBytes: buffer_wma.DecodedPacketCumulativeBytes.as_ptr(),
            PacketCount: u32::try_from(buffer_wma.DecodedPacketCumulativeBytes.len()).map_err(|_| E::INVALIDARG)?
        })
    }
}
