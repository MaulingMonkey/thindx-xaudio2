use super::xaudio2;
use super::xaudio2::sys::*;

use winresult::*;

use core::ptr::null;



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
    /// The following invariants may apply:
    /// *   [XAUDIO2_BUFFER::pContext] may have arbitrary requirements depending on what callbacks have been registered with this voice.
    /// *   [XAUDIO2_BUFFER::pAudioData]\[.. [XAUDIO2_BUFFER::AudioBytes]\] must be valid
    /// *   [XAUDIO2_BUFFER::pAudioData]\[[XAUDIO2_BUFFER::PlayBegin]..\]\[..[XAUDIO2_BUFFER::PlayLength]\] must be valid?
    /// *   [XAUDIO2_BUFFER::pAudioData]\[[XAUDIO2_BUFFER::LoopBegin]..\]\[..[XAUDIO2_BUFFER::LoopLength]\] must be valid?
    /// *   [XAUDIO2_BUFFER::PlayBegin] <= [XAUDIO2_BUFFER::AudioBytes]?
    /// *   [XAUDIO2_BUFFER::LoopBegin] <= [XAUDIO2_BUFFER::AudioBytes]?
    /// *   [XAUDIO2_BUFFER::Flags] may need to be valid?
    ///
    /// And if `buffer_wma` is [Some]:
    /// *   [XAUDIO2_BUFFER_WMA::pDecodedPacketCumulativeBytes]\[.. [XAUDIO2_BUFFER_WMA::PacketCount]\] must be valid
    /// *   [XAUDIO2_BUFFER_WMA::PacketCount] >= 1?
    /// *   [XAUDIO2_BUFFER::AudioBytes] % [XAUDIO2_BUFFER_WMA::PacketCount] == 0?  "Must 'divide evenly'..."
    unsafe fn submit_source_buffer_unchecked(&self, buffer: &XAUDIO2_BUFFER, buffer_wma: Option<&XAUDIO2_BUFFER_WMA>) -> Result<HResultSuccess, HResultError> {
        unsafe { self._as_ixaudio2().SubmitSourceBuffer(buffer, buffer_wma.map_or(null(), |r| r)) }.succeeded()
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
