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
