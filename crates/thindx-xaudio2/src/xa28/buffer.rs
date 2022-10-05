#[allow(unused_imports)] use super::*;
#[allow(unused_imports)] use super::xaudio2::sys::*;

/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/ns-xaudio2-xaudio2_buffer)\] [XAUDIO2_BUFFER]
#[allow(non_snake_case)] // XXX: is this really what we want?
#[derive(Clone, Copy, Debug, Default)] #[repr(C)] pub struct Buffer<'a, Context: Send + Sync + Sized + 'static> {
    /// Either 0 or [XAUDIO2_END_OF_STREAM].
    pub Flags: u32,

    /// Pointer to the audio data buffer.
    pub AudioData: &'a [u8],

    /// First sample in this buffer to be played.
    pub PlayBegin: u32,

    /// Length of the region to be played in samples, or 0 to play the whole buffer.
    pub PlayLength: u32,

    /// First sample of the region to be looped.
    pub LoopBegin: u32,

    /// Length of the desired loop region in samples, or 0 to loop the entire buffer.
    pub LoopLength: u32,

    /// Number of times to repeat the loop region, or XAUDIO2_LOOP_INFINITE to loop forever.
    pub LoopCount: u32,

    /// Context value to be passed back in callbacks.
    pub Context: Context,
}
