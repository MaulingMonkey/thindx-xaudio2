#[allow(unused_imports)] use super::*;
#[allow(unused_imports)] use super::xaudio2::sys::*;
use core::ffi::c_void;



/// Owner of:
/// *   [XAUDIO2_BUFFER::pData]
/// *   [XAUDIO2_BUFFER_WMA::pDecodedPacketCumulativeBytes] (optional)
/// *   [VoiceCallback::BufferContext] parameters
pub(crate) struct SourceBuffer<Context: Send + Sync + Sized + 'static> {
    pub(crate) context:     Context,
    pub(crate) audio_data:  *const c_void,
    pub(crate) audio_len:   usize,
    pub(crate) audio_free:  unsafe fn(*const c_void, usize),
}
