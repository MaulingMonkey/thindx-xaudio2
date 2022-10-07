use super::*;

use core::marker::PhantomData;
use core::ops::*;
use core::ptr::NonNull;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2sourcevoice)\] [IXAudio2SourceVoice]
pub struct SourceVoice<'xa2, Context: Send + Sync + Sized + 'static> {
    factory:    PhantomData<&'xa2 IXAudio2>,
    voice:      NonNull<IXAudio2SourceVoiceTyped<Context>>,
}

impl<'xa2, Context: Send + Sync + Sized + 'static> SourceVoice<'xa2, Context> {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-destroyvoice)\]
    /// Destroys this voice, stopping it if necessary and removing it from the XAudio2 graph.
    ///
    /// (Dropping the voice also implicitly stops/removes it.)
    pub fn destroy_voice(self) {}
    // "It is invalid to call DestroyVoice from within a callback (that is, IXAudio2EngineCallback or IXAudio2VoiceCallback)."
    // Currently this cannot happen as Self : !Send.
    // Consider using https://docs.rs/static_assertions/latest/static_assertions/macro.assert_not_impl_any.html to verify this.
    // See also: util::xaudio2_thread_guard

    /// Create a voice wrapper from a raw pointer.
    ///
    /// If `raw` is null, will return [None].
    ///
    /// ### Safety
    /// *   `raw` must be a valid interface pointer if not null.
    /// *   `Self` takes ownership of `raw`.
    pub unsafe fn from_raw_opt(_xa2: &'xa2 IXAudio2, raw: *const IXAudio2SourceVoiceTyped<Context>) -> Option<Self> { Some(Self {
        factory:    PhantomData,
        voice:      NonNull::new(raw as *mut _)?,
    })}

    /// Create a voice wrapper from a raw pointer.
    ///
    /// If `raw` is null, will [panic!].
    ///
    /// ### Safety
    /// *   `raw` must be a valid interface pointer if not null.
    /// *   `Self` takes ownership of `raw`.
    #[track_caller] pub unsafe fn from_raw(xa2: &'xa2 IXAudio2, raw: *const IXAudio2SourceVoiceTyped<Context>) -> Self { unsafe { Self::from_raw_opt(xa2, raw) }.unwrap() }

    /// Convert `self` back into a raw pointer, relinquishing ownership.
    pub fn into_raw(self) -> *const IXAudio2SourceVoiceTyped<Context> {
        let ptr = self.voice.as_ptr();
        core::mem::forget(self);
        ptr
    }

    pub fn as_raw(&self) -> *const IXAudio2SourceVoiceTyped<Context> { self.voice.as_ptr() }
}

impl<'xa2, Context: Send + Sync + Sized + 'static> Deref      for SourceVoice<'xa2, Context> { fn deref    (&    self) -> &    Self::Target { unsafe { self.voice.as_ref() } } type Target = IXAudio2SourceVoiceTyped<Context>; }
impl<'xa2, Context: Send + Sync + Sized + 'static> DerefMut   for SourceVoice<'xa2, Context> { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { self.voice.as_mut() } } }
impl<'xa2, Context: Send + Sync + Sized + 'static> Drop       for SourceVoice<'xa2, Context> { fn drop(&mut self) { unsafe { (*self.voice.as_ptr()).DestroyVoice() } } }
