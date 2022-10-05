use super::xaudio2::sys::*;

use winresult::{HResultError, E};



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
