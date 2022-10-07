#[allow(unused_imports)] use super::*;
#[allow(unused_imports)] use super::xaudio2::sys::*;
use std::sync::Arc;



/// Owner of:
/// *   [XAUDIO2_BUFFER::pData]
/// *   [XAUDIO2_BUFFER_WMA::pDecodedPacketCumulativeBytes] (optional)
/// *   [VoiceCallback::BufferContext] parameters
pub(crate) struct SourceBuffer<Context: Send + Sync + Sized + 'static> {
    #[allow(dead_code)] pub(crate) audio_data: Arc<[f32]>, // field required to keep audio data alive / avoid use-after-free
    pub(crate) context: Context,
}
