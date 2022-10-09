#[allow(unused_imports)] use super::*;
#[allow(unused_imports)] use super::xaudio2::sys::*;
use std::any::Any;



/// Owner of:
/// *   [XAUDIO2_BUFFER::pData]
/// *   [XAUDIO2_BUFFER_WMA::pDecodedPacketCumulativeBytes] (optional)
/// *   [VoiceCallback::BufferContext] parameters
pub(crate) struct SourceBuffer<Context: Send + Sync + Sized + 'static> {
    pub(crate) context:     Context,
    pub(crate) _audio_data: Box<dyn Any + Send>, // for keepalive only
}
