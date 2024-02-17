use super::xaudio2;

use winresult::*;



impl xaudio2::MasteringVoice<'_> {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2masteringvoice-getchannelmask)\]
    /// Returns the channel mask for this voice.
    ///
    /// This corresponds to the dwChannelMask member of [WAVEFORMATEXTENSIBLE].
    ///
    /// [WAVEFORMATEXTENSIBLE]: https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformatextensible
    pub fn get_channel_mask(&self) -> Result<u32, HResultError> {
        let mut mask = 0;
        unsafe { self.as_ref().GetChannelMask(&mut mask) }.succeeded()?;
        Ok(mask)
    }
}
