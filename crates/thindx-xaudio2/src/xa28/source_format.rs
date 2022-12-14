#[allow(unused_imports)] use super::*;

use winapi::shared::mmreg::*;

use core::marker::PhantomData;
use core::mem::size_of;
use core::ops::*;



/// A [WAVEFORMATEX] or [WAVEFORMATEXTENSIBLE] suitable for passing to [IXAudio2::CreateSourceVoice].
///
/// XAudio2 supports the following PCM formats:
/// *   8-bit (unsigned) integer PCM
/// *   16-bit integer PCM (optimal format for XAudio2)
/// *   20-bit integer PCM (either in 24 or 32 bit containers)
/// *   24-bit integer PCM (either in 24 or 32 bit containers)
/// *   32-bit integer PCM
/// *   32-bit float PCM (preferred format after 16-bit integer)
///
/// [WAVEFORMATEX]:                 https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformatex
/// [WAVEFORMATEXTENSIBLE]:         https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ksmedia/ns-ksmedia-waveformatextensible
/// [IXAudio2::CreateSourceVoice]:  https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2-createsourcevoice
#[derive(Clone, Copy)] pub struct SourceFormat(WAVEFORMATEX);

impl AsRef<SourceFormat> for SourceFormat { fn as_ref(&self) -> &SourceFormat { self } }

impl SourceFormat {
    /// Construct [SourceFormat] from a [WAVEFORMATEX](https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformatex).
    ///
    /// ### Safety
    /// The exact safety requirements are a bit murky.  However:
    /// *   `WAVEFORMATEX::cbSize`      in particular must be valid to avoid buffer overruns.
    /// *   `WAVEFORMATEX::wFormatTag`  should be valid for `self` to avoid buffer overruns / unhandled switch cases.
    /// *   Particularly large integers could easily lead to integer overflow related undefined behavior.
    /// *   Unexpected enum values could easily lead to exercising undefined behavior via unhandled switch cases.
    pub unsafe fn from_wave_format_ex(format: WAVEFORMATEX) -> Self {
        assert!(format.wFormatTag != WAVE_FORMAT_EXTENSIBLE, "use WAVEFORMATEXTENSIBLE instead for WAVE_FORMAT_EXTENSIBLE");
        assert!(format.cbSize == 0, "WAVEFORMATEX cannot store any trailing data");
        Self(format)
    }

    /// [IXAudio2::CreateSourceVoice]-friendly parameter.
    pub fn as_source_format(&self) -> *const WAVEFORMATEX { &self.0 }

    fn basic<S: Sized, const C: usize>(fmt: u16, hz: u32) -> TypedSourceFormat<[S; C]> {
        let sc_size = if let Ok(n) = u16::try_from(size_of::<[S; C]>()) { n } else { panic!("size_of::<[S; C]>() > u16::MAX") };
        let s_size  = if let Ok(n) = u16::try_from(size_of::< S    >()) { n } else { panic!("size_of::<S>() > u16::MAX") };
        unsafe{TypedSourceFormat::new(SourceFormat::from_wave_format_ex(WAVEFORMATEX{
            wFormatTag:         fmt,
            nChannels:          C.try_into().expect("too many channels"),
            nSamplesPerSec:     hz,
            nAvgBytesPerSec:    hz * (sc_size as u32),
            nBlockAlign:        sc_size,
            wBitsPerSample:     s_size * 8,
            cbSize:             0,
        }))}
    }
}

// TODO: impl Debug for SourceFormat



/// [SourceFormat], but with additional type information.
#[derive(Clone, Copy)] pub struct TypedSourceFormat<Sample>(SourceFormat, PhantomData<Sample>);

impl<S> AsRef<SourceFormat> for TypedSourceFormat<S> { fn as_ref(&self) -> &SourceFormat { &self.0 } }
impl<S> Deref for TypedSourceFormat<S> { fn deref(&self) -> &Self::Target { &self.0 } type Target = SourceFormat; }

impl<S> TypedSourceFormat<S> {
    /// Construct a [TypedSourceFormat] from a [SourceFormat]
    ///
    /// ### Safety
    /// *   `format` should match the type `S`.
    pub unsafe fn new(format: SourceFormat) -> Self {
        Self(format, PhantomData)
    }
}

impl<S: HasPcmWaveFormat, const C: usize> TypedSourceFormat<[S; C]> {
    /// [Pulse-code modulation](https://en.wikipedia.org/wiki/Pulse-code_modulation) format, implemented for [TypedSourceFormat]<\[[u8] | [i16] | [i32] | [f32]; 1 \| 2\]>.
    ///
    /// N.B. 8-bit is unsigned, but 16 and 32 bit are *signed*
    pub fn pcm(hz: u32) -> Self { SourceFormat::basic(S::pcm_wave_format(), hz) }
}

/// [u8] | [i16] | [i32] | [f32]
///
/// N.B. 8-bit is unsigned, but 16 and 32 bit are *signed*
pub unsafe trait HasPcmWaveFormat {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformatex#remarks)\]
    /// WAVE_FORMAT_\* tag values
    fn pcm_wave_format() -> u16;
}

unsafe impl HasPcmWaveFormat for u8     { fn pcm_wave_format() -> u16 { WAVE_FORMAT_PCM } }
unsafe impl HasPcmWaveFormat for i16    { fn pcm_wave_format() -> u16 { WAVE_FORMAT_PCM } }
unsafe impl HasPcmWaveFormat for i32    { fn pcm_wave_format() -> u16 { WAVE_FORMAT_PCM } }
unsafe impl HasPcmWaveFormat for f32    { fn pcm_wave_format() -> u16 { WAVE_FORMAT_IEEE_FLOAT } }
