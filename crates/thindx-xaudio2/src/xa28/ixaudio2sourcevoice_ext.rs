use super::*;
use super::xaudio2::sys::*;

use winresult::*;



impl IXAudio2SourceVoiceExt for IXAudio2SourceVoice { fn _as_ixaudio2(&self) -> &IXAudio2SourceVoice { self } }
impl<'xa2                                                                               > IXAudio2SourceVoiceExt for xaudio2::SourceVoiceUntyped<'xa2>           { fn _as_ixaudio2(&self) -> &IXAudio2SourceVoice { self } }
impl<'xa2,                                        Context: Send + Sync + Sized + 'static> IXAudio2SourceVoiceExt for xaudio2::SourceVoiceDynamic<'xa2, Context>  { fn _as_ixaudio2(&self) -> &IXAudio2SourceVoice { self } }
impl<'xa2, Sample: Send + Sync + Sized + 'static, Context: Send + Sync + Sized + 'static> IXAudio2SourceVoiceExt for xaudio2::SourceVoice<'xa2, Sample, Context> { fn _as_ixaudio2(&self) -> &IXAudio2SourceVoice { self } }

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
