use super::*;

use winresult::*;



impl xaudio2::SourceVoiceUntyped<'_> {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-start)\]
    /// Makes this voice start consuming and processing audio.
    pub fn start(&self, flags: u32, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self.as_ref().Start(flags, operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-stop)\]
    /// Makes this voice stop consuming audio.
    pub fn stop(&self, flags: u32, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self.as_ref().Stop(flags, operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-flushsourcebuffers)\]
    /// Removes all pending audio buffers from this voice's queue.
    pub fn flush_source_buffers(&self) -> Result<HResultSuccess, HResultError> {
        unsafe { self.as_ref().FlushSourceBuffers() }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-discontinuity)\]
    /// Notifies the voice of an intentional break in the stream of audio buffers (e.g. the end of a sound), to prevent XAudio2 from interpreting an empty buffer queue as a glitch.
    pub fn discontinuity(&self) -> Result<HResultSuccess, HResultError> {
        unsafe { self.as_ref().Discontinuity() }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-exitloop)\]
    /// Breaks out of the current loop when its end is reached.
    pub fn exit_loop(&self, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self.as_ref().ExitLoop(operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-getstate)\]
    /// Returns the number of buffers currently queued on this voice, the pContext value associated with the currently processing buffer (if any), and other voice state information.
    pub fn get_state(&self, flags: u32) -> xaudio2::VoiceState {
        let mut state = Default::default();
        unsafe { self.as_ref().GetState(&mut state, flags) };
        state
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-setfrequencyratio)\]
    /// Sets this voice's frequency adjustment, i.e. its pitch.
    pub fn set_frequency_ratio(&self, ratio: f32, operation_set: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self.as_ref().SetFrequencyRatio(ratio, operation_set) }.succeeded()
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-getfrequencyratio)\]
    /// Returns this voice's current frequency adjustment ratio.
    pub fn get_frequency_ratio(&self) -> f32 {
        let mut ratio = 0.0;
        unsafe { self.as_ref().GetFrequencyRatio(&mut ratio) };
        ratio
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-setsourcesamplerate)\]
    /// Reconfigures this voice to treat its source data as being at a different sample rate than the original one specified in [IXAudio2::CreateSourceVoice]'s pSourceFormat argument.
    pub fn set_source_sample_rate(&self, new_source_sample_rate: u32) -> Result<HResultSuccess, HResultError> {
        unsafe { self.as_ref().SetSourceSampleRate(new_source_sample_rate) }.succeeded()
    }
}
