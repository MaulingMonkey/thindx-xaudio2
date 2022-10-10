use super::*;
use super::xaudio2::*;
use super::xaudio2::sys::*;

use winresult::*;

use core::marker::PhantomData;
use core::mem::*;
use core::ops::*;
use core::ptr::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2sourcevoice)\] [IXAudio2SourceVoice]
#[repr(transparent)] pub struct SourceVoiceDynamic<'xa2, Context: Send + Sync + Sized + 'static> {
    factory:    PhantomData<&'xa2 IXAudio2>,
    voice:      NonNull<IXAudio2SourceVoice>,
    phantom:    PhantomData<fn (&Context)>,
}

impl<'xa2, Context: Send + Sync + Sized + 'static> SourceVoiceDynamic<'xa2, Context> {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-destroyvoice)\]
    /// Destroys this voice, stopping it if necessary and removing it from the XAudio2 graph.
    ///
    /// (Dropping the voice also implicitly stops/removes it.)
    pub fn destroy_voice(self) {}
    // "It is invalid to call DestroyVoice from within a callback (that is, IXAudio2EngineCallback or IXAudio2VoiceCallback)."
    // Currently this cannot happen as Self : !Send.
    // Consider using https://docs.rs/static_assertions/latest/static_assertions/macro.assert_not_impl_any.html to verify this.
    // See also: util::xaudio2_thread_guard

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2sourcevoice-submitsourcebuffer)\]
    /// Adds a new audio buffer to this voice's input queue.
    ///
    /// ### Safety
    /// It is up to the caller to ensure `audio_data` correctly matches the source voice's format.
    /// Mismatches in sample rate are likely fairly harmless (resulting in speed+pitch up/down).
    /// However, getting the sample *type* wrong could lead to much more sinister outcomes involving undefined behavior.
    pub unsafe fn submit_source_buffer_blob_unchecked<AudioData: AsRef<[u8]> + Send + Sized + 'static>(
        &self,
        flags:          u32,
        audio_data:     Box<AudioData>,
        play_range:     impl Into<xaudio2::SampleRange>,
        loop_range:     impl Into<xaudio2::SampleRange>,
        loop_count:     impl Into<xaudio2::LoopCount>,
        context:        Context,
    ) -> Result<HResultSuccess, HResultError> {
        let play_range  = play_range.into();
        let loop_range  = loop_range.into();
        let loop_count  = loop_count.into();

        let mut b = XAUDIO2_BUFFER {
            Flags:      flags,
            AudioBytes: size_of_val(&audio_data.as_ref().as_ref()[..]).try_into().map_err(|_| E::INVALIDARG)?,
            pAudioData: audio_data.as_ref().as_ref().as_ptr().cast(),
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
            _audio_data: audio_data,
        })).cast();

        unsafe { self.SubmitSourceBuffer(&b, null()) }.succeeded()
    }

    pub fn as_raw(&self) -> *const IXAudio2SourceVoice { self.voice.as_ptr() }

    /// Create a voice wrapper from a raw pointer.
    ///
    /// If `raw` is null, will return [None].
    ///
    /// ### Safety
    /// *   `raw` must be a valid interface pointer if not null.
    /// *   `Self` takes ownership of `raw`.
    pub(crate) unsafe fn from_raw_opt(_xa2: &'xa2 IXAudio2, raw: *const IXAudio2SourceVoice) -> Option<Self> { Some(Self {
        factory:    PhantomData,
        voice:      NonNull::new(raw as *mut _)?,
        phantom:    PhantomData,
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
    pub(crate) fn into_raw(self) -> *const IXAudio2SourceVoice {
        let ptr = self.voice.as_ptr();
        core::mem::forget(self);
        ptr
    }
}

impl<'xa2, Context: Send + Sync + Sized + 'static> From<SourceVoiceDynamic<'xa2, Context>> for SourceVoiceUntyped<'xa2> { fn from(voice: SourceVoiceDynamic<'xa2, Context>) -> Self { unsafe { core::mem::transmute(voice) } }}
impl<'xa2, Context: Send + Sync + Sized + 'static> Deref      for SourceVoiceDynamic<'xa2, Context> { fn deref    (&    self) -> &    Self::Target { unsafe { core::mem::transmute(self) } } type Target = xaudio2::SourceVoiceUntyped<'xa2>; }
impl<'xa2, Context: Send + Sync + Sized + 'static> DerefMut   for SourceVoiceDynamic<'xa2, Context> { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { core::mem::transmute(self) } } }
impl<'xa2, Context: Send + Sync + Sized + 'static> Drop       for SourceVoiceDynamic<'xa2, Context> { fn drop(&mut self) { unsafe { (*self.voice.as_ptr()).DestroyVoice() } } }
