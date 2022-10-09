use super::*;
use super::xaudio2::sys::*;

use winresult::*;

use std::sync::Arc;

use core::marker::PhantomData;
use core::mem::size_of_val;
use core::ptr::null;



/// [IXAudio2SourceVoice], but with a typed context for callbacks / submitted source buffers.
#[repr(transparent)] pub struct IXAudio2SourceVoiceTyped<Sample: Send + Sync + Sized + 'static, Context: Send + Sync + Sized + 'static>(IXAudio2SourceVoice, PhantomData<(Sample,Context)>);
impl<Sample: Send + Sync + Sized + 'static, Context: Send + Sync + Sized + 'static> core::ops::Deref for IXAudio2SourceVoiceTyped<Sample, Context> { type Target = IXAudio2SourceVoice; fn deref(&self) -> &Self::Target { &self.0 } }
impl<Sample: Send + Sync + Sized + 'static, Context: Send + Sync + Sized + 'static> IXAudio2SourceVoiceTyped<Sample, Context> {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-submitsourcebuffer)\]
    /// Adds a new audio buffer to this voice's input queue.
    pub fn submit_source_buffer(
        &self,
        flags:          u32,
        audio_data:     impl Into<Arc<[Sample]>>,
        play_range:     impl Into<xaudio2::SampleRange>,
        loop_range:     impl Into<xaudio2::SampleRange>,
        loop_count:     impl Into<xaudio2::LoopCount>,
        context:        Context,
    ) -> Result<HResultSuccess, HResultError> {
        let audio_data  = audio_data.into();
        let play_range  = play_range.into();
        let loop_range  = loop_range.into();
        let loop_count  = loop_count.into();

        let mut b = XAUDIO2_BUFFER {
            Flags:      flags,
            AudioBytes: size_of_val(&audio_data[..]).try_into().map_err(|_| E::INVALIDARG)?,
            pAudioData: audio_data.as_ptr().cast(),
            .. Default::default()
        };

        match play_range.into_raw_xaudio2_begin_length() {
            None                    => return Ok(S::OK),
            Some((begin, length))   => {
                b.PlayBegin     = begin;
                b.PlayLength    = length;
            },
        }

        if loop_count.0 != 0 {
            if let Some((begin, length)) = loop_range.into_raw_xaudio2_begin_length() {
                b.LoopBegin     = begin;
                b.LoopLength    = length;
                b.LoopCount     = loop_count.0.into();
            }
        }

        b.pContext = Box::into_raw(Box::new(SourceBuffer::<Context> {
            context,
            _audio_data: Box::new(audio_data),
        })).cast();

        unsafe { self.SubmitSourceBuffer(&b, null()) }.succeeded()
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
