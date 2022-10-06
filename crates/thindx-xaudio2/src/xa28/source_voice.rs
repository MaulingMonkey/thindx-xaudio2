use super::*;

use core::ops::*;
use core::ptr::NonNull;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2sourcevoice)\] [IXAudio2SourceVoice]
pub struct SourceVoice<Context: Send + Sync + Sized + 'static> {
    voice:      NonNull<IXAudio2SourceVoiceTyped<Context>>,
    callback:   Box<dyn Deref<Target = IXAudio2VoiceCallback>>,
}

impl<Context: Send + Sync + Sized + 'static> SourceVoice<Context> {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/desktop/api/xaudio2/nf-xaudio2-ixaudio2voice-destroyvoice)\]
    /// Syncronously destroys the voice, stopping it if necessary and removing it from the XAudio2 graph.
    ///
    /// (Dropping the voice also implicitly stops/removes it.)
    pub fn destroy_voice_sync(self) {}

    // TODO: destroy_voice_async?

    /// Create a voice wrapper from a raw pointer.
    ///
    /// If `voice` is null, will [panic!].
    ///
    /// ### Safety
    /// *   `voice` must be a valid interface pointer if not null.
    /// *   `Self` takes ownership of `voice`.
    #[track_caller] pub(crate) unsafe fn new(voice: *const IXAudio2SourceVoiceTyped<Context>, callback: Box<dyn Deref<Target = IXAudio2VoiceCallback>>) -> Self {
        let voice = NonNull::new(voice as *mut _).expect("voice cannot be null");
        Self { voice, callback }
    }
}

impl<Context: Send + Sync + Sized + 'static> Deref      for SourceVoice<Context> { fn deref    (&    self) -> &    Self::Target { unsafe { self.voice.as_ref() } } type Target = IXAudio2SourceVoiceTyped<Context>; }
impl<Context: Send + Sync + Sized + 'static> DerefMut   for SourceVoice<Context> { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { self.voice.as_mut() } } }

impl<Context: Send + Sync + Sized + 'static> Drop for SourceVoice<Context> {
    fn drop(&mut self) {
        // This is syncronous, and must occur before self.callback is dropped.
        // XAudio2 threads might UAF (Use After Free) said callbacks otherwise.
        unsafe { (*self.voice.as_ptr()).DestroyVoice() };

        // allow self.callback to drop implicitly after Destroy
    }
}
