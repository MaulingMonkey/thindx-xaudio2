use super::XAudio2;
use super::xaudio2;
use super::xaudio2::sys::*;

use winresult::{E, HResultError};



/// \[<strike>microsoft.com</strike>\] IXAudio2Extension
/// &mdash; XAudio 2.9 extension methods to IXAudio2
///
/// ### Example
/// ```
/// use thindx_xaudio2::xaudio2_9::*;
/// let ci = std::env::var_os("CI").is_some();
///
/// let xaudio2 = unsafe { xaudio2::create(None, None) }.expect("xaudio2::create");
/// if let Ok(ext) = xaudio2::Extension::try_from(&xaudio2) {
///     # if ci {
///     #     let _ = ext.get_processing_quantum();
///     #     let _ = ext.get_processor();
///     # } else {
///     // Not actually guaranteed
///     assert_eq!(ext.get_processing_quantum(), (1, 100));
///     assert_eq!(ext.get_processor(), xaudio2::Processor1);
///     # }
/// } else {
///     assert!(ci, "Expected cast to IXAudio2Extension to work on most non-CI boxes");
/// }
/// ```
///
#[derive(Clone)] #[repr(transparent)] pub struct XAudio2Extension(mcom::Rc<IXAudio2Extension>);

impl core::ops::Deref for XAudio2Extension {
    fn deref(&self) -> &Self::Target { &self.0 }
    type Target = mcom::Rc<IXAudio2Extension>;
}

impl TryFrom<&XAudio2> for XAudio2Extension {
    fn try_from(xaudio2: &XAudio2) -> Result<Self, Self::Error> { Ok(Self(xaudio2.try_cast::<IXAudio2Extension>().ok_or(E::NOINTERFACE)?)) }
    type Error = HResultError;
}

impl TryFrom<XAudio2> for XAudio2Extension {
    fn try_from(xaudio2: XAudio2) -> Result<Self, Self::Error> { Self::try_from(&xaudio2) }
    type Error = HResultError;
}

impl XAudio2Extension {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable#duration-of-audio-processing-quantum)\]
    /// Returns the processing quantum as a ratio of seconds (e.g. `(1, 100)` = `10ms`)
    pub fn get_processing_quantum(&self) -> (u32, u32) {
        let mut n = 0;
        let mut d = 0;
        unsafe { self.GetProcessingQuantum(&mut n, &mut d) };
        (n, d)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable#preferred-cpu-core)\]
    /// Returns the [xaudio2::Processor] used by XAudio2.
    ///
    /// This will generally be the same as the processor passed to [xaudio2::create],
    /// unless you specified [xaudio2::USE_DEFAULT_PROCESSOR],
    /// in which case the actual processor(s) chosen by XAudio2 will be returned instead.
    pub fn get_processor(&self) -> xaudio2::Processor {
        let mut p = xaudio2::Processor::default();
        unsafe { self.GetProcessor(&mut p) };
        p
    }
}
