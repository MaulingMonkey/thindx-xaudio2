use super::xaudio2::sys::*;

use winresult::*;



impl IXAudio2MasteringVoiceExt for IXAudio2MasteringVoice { fn _as_ixaudio2(&self) -> &IXAudio2MasteringVoice { self } }

/// [IXAudio2MasteringVoice] extension methods
pub trait IXAudio2MasteringVoiceExt {
    #[doc(hidden)] fn _as_ixaudio2(&self) -> &IXAudio2MasteringVoice;

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2masteringvoice-getchannelmask)\]
    /// Returns the channel mask for this voice.
    ///
    /// This corresponds to the dwChannelMask member of [WAVEFORMATEXTENSIBLE].
    ///
    /// [WAVEFORMATEXTENSIBLE]: https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformatextensible
    fn get_channel_mask(&self) -> Result<u32, HResultError> {
        let mut mask = 0;
        unsafe { self._as_ixaudio2().GetChannelMask(&mut mask) }.succeeded()?;
        Ok(mask)
    }
}
