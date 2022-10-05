use super::xaudio2;
use super::xaudio2::sys::*;

use winresult::*;

use core::marker::PhantomData;
use core::mem::size_of;
use core::ptr::{null, NonNull};



/// [IXAudio2SourceVoice], but with a typed context for callbacks / submitted source buffers.
#[repr(transparent)] pub struct IXAudio2SourceVoiceTyped<Context: Send + Sync + Sized + 'static>(IXAudio2SourceVoice, PhantomData<Context>);
impl<Context: Send + Sync + Sized + 'static> core::ops::Deref for IXAudio2SourceVoiceTyped<Context> { type Target = IXAudio2SourceVoice; fn deref(&self) -> &Self::Target { &self.0 } }
impl<Context: Send + Sync + Sized + 'static> IXAudio2SourceVoiceTyped<Context> {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-submitsourcebuffer)\]
    /// Adds a new audio buffer to this voice's input queue.
    pub fn submit_source_buffer(&self, buffer: xaudio2::Buffer<Context>, buffer_wma: Option<&xaudio2::BufferWma>) -> Result<HResultSuccess, HResultError> {
        let buffer = XAUDIO2_BUFFER {
            Flags:      buffer.Flags,
            AudioBytes: buffer.AudioData.len().try_into().map_err(|_| E::INVALIDARG)?,
            pAudioData: buffer.AudioData.as_ptr(),
            LoopBegin:  buffer.LoopBegin,
            LoopCount:  buffer.LoopCount,
            LoopLength: buffer.LoopLength,
            PlayBegin:  buffer.PlayBegin,
            PlayLength: buffer.PlayLength,
            // XXX: Consider replacing `Box` with trait driven `trait UserData { into_raw, from_raw, borrow_raw }`
            pContext:   if size_of::<Context>() == 0 {
                // Allow the likes of IXAudio2Ext::create_source_voice_no_callback to use xaudio2::SourceVoice<()> without allocating/freeing boxes
                NonNull::dangling().as_ptr()
            } else {
                Box::into_raw(Box::new(buffer.Context)).cast()
            },
        };
        unsafe { self.submit_source_buffer_unchecked(&buffer, buffer_wma) }
    }
}



impl IXAudio2SourceVoiceExt for IXAudio2SourceVoice { fn _as_ixaudio2(&self) -> &IXAudio2SourceVoice { self } }

/// [IXAudio2SourceVoice] extension methods
pub trait IXAudio2SourceVoiceExt {
    #[doc(hidden)] fn _as_ixaudio2(&self) -> &IXAudio2SourceVoice;

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-start)\]
    /// Makes this voice start consuming and processing audio.
    fn start(&self, flags: u32, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self._as_ixaudio2().Start(flags, operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-stop)\]
    /// Makes this voice stop consuming audio.
    fn stop(&self, flags: u32, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self._as_ixaudio2().Stop(flags, operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-submitsourcebuffer)\]
    /// Adds a new audio buffer to this voice's input queue.
    ///
    /// ### Safety
    /// The requirements imposed are numerous.  I strongly recommend carefully reading through at least:
    /// *   [IXAudio2SourceVoice::SubmitSourceBuffer: Remarks](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-submitsourcebuffer#remarks)
    /// *   [XAUDIO2_BUFFER](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer): [Members](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer#members) + [Remarks](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer#remarks)
    /// *   [XAUDIO2_BUFFER_WMA](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer_wma): [Members](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer_wma#members) + [Remarks](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer_wma#remarks)
    ///
    /// A **partial** list of requirements that may impact soundness includes:
    /// *   [XAUDIO2_BUFFER::pContext] may have arbitrary requirements imposed upon it by [IXAudio2VoiceCallback].
    /// *   [XAUDIO2_BUFFER::pAudioData] must remain valid until [IXAudio2VoiceCallback::OnBufferEnd] for the ranges:
    ///     *   (pAudioData as *const u8)\[.. [XAUDIO2_BUFFER::AudioBytes]\]
    ///     *   (pAudioData as *const Sample)\[[XAUDIO2_BUFFER::PlayBegin]..\]\[..[XAUDIO2_BUFFER::PlayLength]\]
    ///     *   (pAudioData as *const Sample)\[[XAUDIO2_BUFFER::LoopBegin]..\]\[..[XAUDIO2_BUFFER::LoopLength]\]
    /// *   [XAUDIO2_BUFFER::PlayBegin] <= [XAUDIO2_BUFFER::AudioBytes]?
    /// *   [XAUDIO2_BUFFER::LoopBegin] <= [XAUDIO2_BUFFER::AudioBytes]?
    /// *   [XAUDIO2_BUFFER::Flags] may need to be valid?
    ///
    /// And if `buffer_wma` is [Some]:
    /// *   [XAUDIO2_BUFFER_WMA::pDecodedPacketCumulativeBytes] must remain valid until [IXAudio2VoiceCallback::OnBufferEnd]:
    ///     *   pDecodedPacketCumulativeBytes\[.. [XAUDIO2_BUFFER_WMA::PacketCount]\] must be valid
    ///     *   "byte swapped when loading the buffer on Xbox 360" (big endian?)
    /// *   [XAUDIO2_BUFFER_WMA::PacketCount] >= 1?
    /// *   [XAUDIO2_BUFFER::AudioBytes] % [XAUDIO2_BUFFER_WMA::PacketCount] == 0?  "Must 'divide evenly'..."
    unsafe fn submit_source_buffer_unchecked(&self, buffer: &XAUDIO2_BUFFER, buffer_wma: Option<&xaudio2::BufferWma>) -> Result<HResultSuccess, HResultError> {
        let buffer_wma = buffer_wma.map(|bw| XAUDIO2_BUFFER_WMA::try_from(*bw)).transpose()?;
        unsafe { self._as_ixaudio2().SubmitSourceBuffer(buffer, buffer_wma.as_ref().map_or(null(), |r| r)) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-flushsourcebuffers)\]
    /// Removes all pending audio buffers from this voice's queue.
    fn flush_source_buffers(&self) -> Result<HResultSuccess, HResultError> {
        unsafe { self._as_ixaudio2().FlushSourceBuffers() }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-discontinuity)\]
    /// Notifies the voice of an intentional break in the stream of audio buffers (e.g. the end of a sound), to prevent XAudio2 from interpreting an empty buffer queue as a glitch.
    fn discontinuity(&self) -> Result<HResultSuccess, HResultError> {
        unsafe { self._as_ixaudio2().Discontinuity() }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-exitloop)\]
    /// Breaks out of the current loop when its end is reached.
    fn exit_loop(&self, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self._as_ixaudio2().ExitLoop(operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-getstate)\]
    /// Returns the number of buffers currently queued on this voice, the pContext value associated with the currently processing buffer (if any), and other voice state information.
    fn get_state(&self, flags: u32) -> xaudio2::VoiceState {
        let mut state = Default::default();
        unsafe { self._as_ixaudio2().GetState(&mut state, flags) };
        state
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-setfrequencyratio)\]
    /// Sets this voice's frequency adjustment, i.e. its pitch.
    fn set_frequency_ratio(&self, ratio: f32, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self._as_ixaudio2().SetFrequencyRatio(ratio, operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-getfrequencyratio)\]
    /// Returns this voice's current frequency adjustment ratio.
    fn get_frequency_ratio(&self) -> f32 {
        let mut ratio = 0.0;
        unsafe { self._as_ixaudio2().GetFrequencyRatio(&mut ratio) };
        ratio
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-setsourcesamplerate)\]
    /// Reconfigures this voice to treat its source data as being at a different sample rate than the original one specified in [IXAudio2::CreateSourceVoice]'s pSourceFormat argument.
    fn set_source_sample_rate(&self, new_source_sample_rate: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self._as_ixaudio2().SetSourceSampleRate(new_source_sample_rate) }.succeeded()
    }
}