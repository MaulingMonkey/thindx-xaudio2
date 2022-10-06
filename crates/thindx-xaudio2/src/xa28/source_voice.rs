use super::*;

use core::ops::*;
use core::ptr::NonNull;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2sourcevoice)\] [IXAudio2SourceVoice]
pub struct SourceVoice<Context: Send + Sync + Sized + 'static>(NonNull<IXAudio2SourceVoiceTyped<Context>>);

impl<Context: Send + Sync + Sized + 'static> SourceVoice<Context> {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-destroyvoice)\]
    /// Destroys this voice, stopping it if necessary and removing it from the XAudio2 graph.
    ///
    /// (Dropping the voice also implicitly stops/removes it.)
    pub fn destroy_voice(self) {}

    /// Create a voice wrapper from a raw pointer.
    ///
    /// If `raw` is null, will return [None].
    ///
    /// ### Safety
    /// *   `raw` must be a valid interface pointer if not null.
    /// *   `Self` takes ownership of `raw`.
    pub unsafe fn from_raw_opt(raw: *const IXAudio2SourceVoiceTyped<Context>) -> Option<Self> { Some(Self(NonNull::new(raw as *mut _)?)) }

    /// Create a voice wrapper from a raw pointer.
    ///
    /// If `raw` is null, will [panic!].
    ///
    /// ### Safety
    /// *   `raw` must be a valid interface pointer if not null.
    /// *   `Self` takes ownership of `raw`.
    #[track_caller] pub unsafe fn from_raw(raw: *const IXAudio2SourceVoiceTyped<Context>) -> Self { unsafe { Self::from_raw_opt(raw) }.unwrap() }

    /// Convert `self` back into a raw pointer, relinquishing ownership.
    pub fn into_raw(self) -> *const IXAudio2SourceVoiceTyped<Context> {
        let ptr = self.0.as_ptr();
        core::mem::forget(self);
        ptr
    }

    pub fn as_raw(&self) -> *const IXAudio2SourceVoiceTyped<Context> { self.0.as_ptr() }
}

impl<Context: Send + Sync + Sized + 'static> Deref      for SourceVoice<Context> { fn deref    (&    self) -> &    Self::Target { unsafe { self.0.as_ref() } } type Target = IXAudio2SourceVoiceTyped<Context>; }
impl<Context: Send + Sync + Sized + 'static> DerefMut   for SourceVoice<Context> { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { self.0.as_mut() } } }
impl<Context: Send + Sync + Sized + 'static> Drop       for SourceVoice<Context> { fn drop(&mut self) { unsafe { (*self.0.as_ptr()).DestroyVoice() } } }
