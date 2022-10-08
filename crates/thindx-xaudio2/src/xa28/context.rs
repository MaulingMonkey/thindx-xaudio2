#[allow(unused_imports)] use super::*;
#[allow(unused_imports)] use super::xaudio2::sys::*;
use std::sync::Arc;
use core::ffi::c_void;
use core::ops::Deref;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer)\] [XAUDIO2_BUFFER::pContext]-compatible type.
///
/// In general, the lifecycle of a [Context] instance goes something like this:
/// -   User creates a [Context] with audio data / context.
/// -   User submits the [Context] to [IXAudio2SourceVoice::SubmitSourceBuffer] via [XAUDIO2_BUFFER::pContext].
/// -   Ownership is [Send]t to an XAudio2 thread or thread pool.
/// -   Various [IXAudio2VoiceCallback] callbacks allow access to the [Deref]erenced object.
/// -   [IXAudio2VoiceCallback::OnBufferEnd] is the final use, and [Drop]s the [Context].
///
/// A typical impl of [Context] will keep alive the data referenced by:
/// *   [XAUDIO2_BUFFER::pAudioData]
/// *   [XAUDIO2_BUFFER_WMA::pDecodedPacketCumulativeBytes] (if any)
pub trait Context
    : Deref     // Context should dereference to *something*
    + Sized     // XAUDIO2_BUFFER::pContext                 is a narrow pointer, can't support DSTs
    + Send      // IXAudio2VoiceCallback::OnBufferEnd       executing on an XAudio2 thread, will presumably [Drop] this type.
    + 'static   // IXAudio2SourceVoice::SubmitSourceBuffer  is asyncronous, Self will outlive the call
    where Self::Target : Sized
{
    /// Transfer ownership of `Self` into a [XAUDIO2_BUFFER::pContext] pointer.
    fn into_pcontext(self) -> *mut c_void;

    /// Transfer ownership of a [XAUDIO2_BUFFER::pContext] pointer back into `Self`.
    ///
    /// ### Safety
    /// *   `pcontext` must be a valid (properly aligned, non-dangling) pointer to a Self raw instance.
    /// *   `pcontext` should no longer be used after calling this function, as ownership has been transfered from it.
    unsafe fn from_pcontext(pcontext: *mut c_void) -> Self;

    /// Borrow a reference to `Self` from a [XAUDIO2_BUFFER::pContext] pointer.
    ///
    /// ### Safety
    /// *   `pcontext` must be a valid (properly aligned, non-dangling) pointer to a Self raw instance.
    /// *   The returned reference cannot have a proper lifetime constraint.
    ///     It is instead 'improperly' constrained by the lifetime of the pointer (which may outlive the pointed-to object.)
    unsafe fn borrow_pcontext(pcontext: &*mut c_void) -> &Self::Target;
}

// XXX: I suspect T:Sync might be overkill for Box?
// However, I don't know if a given [Context] can be accessed from multiple XAudio2 threads or not.
// Better safe than sorry!
impl<T: Sized + Send + Sync + 'static> Context for Box<T> {
    fn into_pcontext(self) -> *mut c_void { Box::into_raw(self).cast() }
    unsafe fn from_pcontext(pcontext: *mut c_void) -> Self { unsafe { Box::from_raw(pcontext as *mut _) } }
    unsafe fn borrow_pcontext(pcontext: &*mut c_void) -> &Self::Target { unsafe { &*(*pcontext).cast() } }
}

// T:Sync *definitely* required here.
impl<T: Sized + Send + Sync + 'static> Context for Arc<T> {
    fn into_pcontext(self) -> *mut c_void { Arc::into_raw(self) as *mut _ }
    unsafe fn from_pcontext(pcontext: *mut c_void) -> Self { unsafe { Arc::from_raw(pcontext.cast()) } }
    unsafe fn borrow_pcontext(pcontext: &*mut c_void) -> &Self::Target { unsafe { &*(*pcontext).cast() } }
}

// T:Sync *definitely* required here.
// It'd be sane to IXAudio2SourceVoice::SubmitSourceBuffer a &'static [f32] if you want a zero alloc/dealloc submit
impl<T: Sized + Sync + 'static> Context for &'static T {
    fn into_pcontext(self) -> *mut c_void { self as *const T as *mut _ }
    unsafe fn from_pcontext(pcontext: *mut c_void) -> Self { unsafe { &*pcontext.cast() } }
    unsafe fn borrow_pcontext(pcontext: &*mut c_void) -> &Self::Target { unsafe { &*(*pcontext).cast() } }
}
