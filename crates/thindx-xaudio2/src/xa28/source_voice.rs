use super::*;
use super::xaudio2::*;
use super::xaudio2::sys::*;

use winresult::*;

use std::sync::Arc;

use core::marker::PhantomData;
use core::mem::*;
use core::ops::*;
use core::ptr::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2sourcevoice)\] [IXAudio2SourceVoice]
///
/// Deref chain: [`SourceVoice`] → [`IXAudio2SourceVoiceExt`]\([`Ext`](IXAudio2SourceVoiceExt)\) → [`IXAudio2Voice`]\([`Ext`](IXAudio2VoiceExt)\)
///
/// ### Methods (on `SourceVoice` itself)
/// | Method                                                                            | Description  |
/// | ----------------------------------------------------------------------------------| -------------|
/// | [`destroy_voice`](Self::destroy_voice)                                            | Destroys this voice, stopping it if necessary and removing it from the XAudio2 graph.
/// | [`submit_source_buffer`](Self::submit_source_buffer)                              | Adds a new audio buffer to this voice's input queue.
///
/// ### Methods (via `IXAudio2SourceVoiceExt` after `Deref`)
/// | Method                                                                            | Description  |
/// | ----------------------------------------------------------------------------------| -------------|
/// | [`start`](IXAudio2SourceVoiceExt::start)                                          | Makes this voice start consuming and processing audio.
/// | [`stop`](IXAudio2SourceVoiceExt::stop)                                            | Makes this voice stop consuming audio.
/// | [`flush_source_buffers`](IXAudio2SourceVoiceExt::flush_source_buffers)            | Removes all pending audio buffers from this voice's queue.
/// | [`discontinuity`](IXAudio2SourceVoiceExt::discontinuity)                          | Notifies the voice of an intentional break in the stream of audio buffers (e.g. the end of a sound), to prevent XAudio2 from interpreting an empty buffer queue as a glitch.
/// | [`exit_loop`](IXAudio2SourceVoiceExt::exit_loop)                                  | Breaks out of the current loop when its end is reached.
/// | [`get_state`](IXAudio2SourceVoiceExt::get_state)                                  | Returns the number of buffers currently queued on this voice, the pContext value associated with the currently processing buffer (if any), and other voice state information.
/// | [`set_frequency_ratio`](IXAudio2SourceVoiceExt::set_frequency_ratio)              | Sets this voice's frequency adjustment, i.e. its pitch.
/// | [`get_frequency_ratio`](IXAudio2SourceVoiceExt::get_frequency_ratio)              | Returns this voice's current frequency adjustment ratio.
/// | [`set_source_sample_rate`](IXAudio2SourceVoiceExt::set_source_sample_rate)        | Reconfigures this voice to treat its source data as being at a different sample rate than the original one specified in [IXAudio2::CreateSourceVoice]'s pSourceFormat argument.
///
/// ### Methods (via `IXAudio2VoiceExt` after `Deref`)
/// | Method                                                                            | Description  |
/// | ----------------------------------------------------------------------------------| -------------|
/// | [`get_voice_details`](IXAudio2VoiceExt::get_voice_details)                        | Get [`VoiceDetails`] (flags, channels, sample rate)
/// | [`set_output_voices`](IXAudio2VoiceExt::set_output_voices)                        | Set submix/mastering voices that receive this voice’s output.
/// | [`set_effect_chain`](IXAudio2VoiceExt::set_effect_chain)                          | Replaces this voice’s current effect chain with a new one.
/// | [`enable_effect`](IXAudio2VoiceExt::enable_effect)                                | Enables an effect in this voice’s effect chain.
/// | [`disable_effect`](IXAudio2VoiceExt::disable_effect)                              | Disables an effect in this voice’s effect chain.
/// | [`get_effect_state`](IXAudio2VoiceExt::get_effect_state)                          | Returns the running state of an effect.
/// | [`set_effect_parameters_raw`](IXAudio2VoiceExt::set_effect_parameters_raw)        | Sets effect-specific parameters.  Not type checked.
/// | [`get_effect_parameters_raw`](IXAudio2VoiceExt::get_effect_parameters_raw)        | Obtains the current effect-specific parameters.  Not type checked.
/// | [`set_filter_parameters`](IXAudio2VoiceExt::set_filter_parameters)                | Sets this voice’s [`FilterParameters`].
/// | [`get_filter_parameters`](IXAudio2VoiceExt::get_filter_parameters)                | Returns this voice’s current [`FilterParameters`].
/// | [`set_output_filter_parameters`](IXAudio2VoiceExt::set_output_filter_parameters)  | Sets the [`FilterParameters`] on one of this voice’s sends.
/// | [`get_output_filter_parameters`](IXAudio2VoiceExt::get_output_filter_parameters)  | Returns the [`FilterParameters`] from one of this voice’s sends.
/// | [`set_volume`](IXAudio2VoiceExt::set_volume)                                      | Sets this voice’s overall volume level.
/// | [`get_volume`](IXAudio2VoiceExt::get_volume)                                      | Obtains this voice’s current overall volume level.
/// | [`set_channel_volumes`](IXAudio2VoiceExt::set_channel_volumes)                    | Sets this voice’s per-channel volume levels.
/// | [`get_channel_volumes`](IXAudio2VoiceExt::get_channel_volumes)                    | Returns this voice’s current per-channel volume levels.
/// | [`set_output_matrix`](IXAudio2VoiceExt::set_output_matrix)                        | Sets the volume levels used to mix from each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
/// | [`get_output_matrix`](IXAudio2VoiceExt::get_output_matrix)                        | Obtains the volume levels used to send each channel of this voice’s output audio to each channel of a given destination voice’s input audio.
#[repr(transparent)] pub struct SourceVoice<'xa2, Sample: Send + Sync + Sized + 'static, Context: Send + Sync + Sized + 'static> {
    voice:      SourceVoiceDynamic<'xa2, Context>,
    context:    PhantomData<fn(&[Sample])>,
}

impl<'xa2, Sample: Send + Sync + Sized + 'static, Context: Send + Sync + Sized + 'static> SourceVoice<'xa2, Sample, Context> {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-destroyvoice)\]
    /// Destroys this voice, stopping it if necessary and removing it from the XAudio2 graph.
    ///
    /// (Dropping the voice also implicitly stops/removes it.)
    pub fn destroy_voice(self) { self.voice.destroy_voice() }

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
        // XXX: enforce with a new `Self` type instead for easier refactoring / compile time avoidance of this assert
        assert!(std::mem::size_of::<Sample>() > 0, "IXAudio2SourceVoiceTyped<S, ...>::submit_source_buffer isn't intended for S : ZST");

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

    /// Create a voice wrapper from a raw pointer.
    ///
    /// If `raw` is null, will return [None].
    ///
    /// ### Safety
    /// *   `raw` must be a valid interface pointer if not null.
    /// *   `Self` takes ownership of `raw`.
    pub(crate) unsafe fn from_raw_opt(_xa2: &'xa2 IXAudio2, raw: *const IXAudio2SourceVoice) -> Option<Self> { Some(Self {
        voice:      unsafe { SourceVoiceDynamic::from_raw_opt(_xa2, raw)? },
        context:    PhantomData,
    })}

    /// Create a voice wrapper from a raw pointer.
    ///
    /// If `raw` is null, will [panic!].
    ///
    /// ### Safety
    /// *   `raw` must be a valid interface pointer if not null.
    /// *   `Self` takes ownership of `raw`.
    #[track_caller] pub(crate) unsafe fn from_raw(xa2: &'xa2 IXAudio2, raw: *const IXAudio2SourceVoice) -> Self { unsafe { Self::from_raw_opt(xa2, raw) }.unwrap() }

    #[allow(dead_code)] // TODO: consider removing this fn outright
    /// Convert `self` back into a raw pointer, relinquishing ownership.
    pub(crate) fn into_raw(self) -> *const IXAudio2SourceVoice { self.voice.into_raw() }
}

impl<'xa2, Sample: Send + Sync + Sized + 'static, Context: Send + Sync + Sized + 'static> From<SourceVoice<'xa2, Sample, Context>> for SourceVoiceDynamic<'xa2, Context> { fn from(voice: SourceVoice<'xa2, Sample, Context>) -> Self { voice.voice }}
impl<'xa2, Sample: Send + Sync + Sized + 'static, Context: Send + Sync + Sized + 'static> Deref      for SourceVoice<'xa2, Sample, Context> { fn deref    (&    self) -> &    Self::Target { &    self.voice } type Target = xaudio2::SourceVoiceDynamic<'xa2, Context>; }
impl<'xa2, Sample: Send + Sync + Sized + 'static, Context: Send + Sync + Sized + 'static> DerefMut   for SourceVoice<'xa2, Sample, Context> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.voice } }
