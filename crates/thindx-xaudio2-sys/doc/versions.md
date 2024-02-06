# XAudio2 Versions

| DLL                                           | Ships With                                                                                                                                                    | Redist                                | Minimum OS                                                                                                                |
| ----------------------------------------------|:-------------------------------------------------------------------------------------------------------------------------------------------------------------:|:-------------------------------------:|:-------------------------------------------------------------------------------------------------------------------------:|
| [XAudio2_9.dll](#xaudio2_9dll)                | [Windows 10](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-29-windows-10-and-redistributable-for-windows-7-and-windows-8x)  | [NuGet Package]                       | [Windows 7 SP1](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable)                          |
| [XAudio2_8.dll](#xaudio2_8dll)                | [Windows 8](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-28-windows-8x)                                                    | <span style="opacity: 50%">N/A</span> | [Windows 8?](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-28-windows-8x)               |
| [XAudio2_7.dll](#xaudio2_7dll-and-earlier)    | <span style="opacity: 50%">Windows 8?</span>                                                                                                                  | [DirectX SDK]                         | [Windows XP?](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-27-and-earlier-windows-7)   |
| XAudio2_6.dll                                 | <span style="opacity: 50%">Windows 8?</span>                                                                                                                  | [DirectX SDK]                         | [Windows XP?](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-27-and-earlier-windows-7)   |
| XAudio2_5.dll                                 | <span style="opacity: 50%">Windows 8?</span>                                                                                                                  | [DirectX SDK]                         | [Windows XP?](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-27-and-earlier-windows-7)   |
| XAudio2_4.dll                                 | <span style="opacity: 50%">Windows 8?</span>                                                                                                                  | [DirectX SDK]                         | [Windows XP?](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-27-and-earlier-windows-7)   |
| XAudio2_3.dll                                 | <span style="opacity: 50%">Windows 8?</span>                                                                                                                  | [DirectX SDK]                         | [Windows XP?](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-27-and-earlier-windows-7)   |
| XAudio2_2.dll                                 | <span style="opacity: 50%">Windows 8?</span>                                                                                                                  | [DirectX SDK]                         | [Windows XP?](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-27-and-earlier-windows-7)   |
| XAudio2_1.dll                                 | <span style="opacity: 50%">Windows 8?</span>                                                                                                                  | [DirectX SDK]                         | [Windows XP?](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-27-and-earlier-windows-7)   |
| XAudio2_0.dll                                 | <span style="opacity: 50%">Windows 8?</span>                                                                                                                  | [DirectX SDK]                         | [Windows XP?](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-27-and-earlier-windows-7)   |



### XAudio2_9.dll

\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-29-windows-10-and-redistributable-for-windows-7-and-windows-8x)\] Breaking Changes:
*   [`IID_IXAudio2`](crate::xaudio2_9::IID_IXAudio2) bumped
*   [`ReverbConvertI3DL2ToNative`](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2fx/nf-xaudio2fx-reverbconverti3dl2tonative) gains a `BOOL sevenDotOneReverb` parameter
*   ...

Additions:
*   Introduced [`IXAudio2Extension`](crate::xaudio2_9::IXAudio2Extension)
*   Introduced [`CreateHrtfApo`](https://learn.microsoft.com/en-us/windows/win32/api/hrtfapoapi/nf-hrtfapoapi-createhrtfapo)
*   New constants for old APIs
*   ...



### XAudio2_8.dll

\[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions#xaudio-28-windows-8x)\] Breaking Changes:
*   [`IID_IXAudio2`](crate::xaudio2_8::IID_IXAudio2) bumped
*   Added <code>[IXAudio2MasteringVoice](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nn-xaudio2-ixaudio2masteringvoice)::[GetChannelMask](https://learn.microsoft.com/en-us/windows/win32/api/xaudio2/nf-xaudio2-ixaudio2masteringvoice-getchannelmask)</code>
*   ...

Additions:
*   ...



### XAudio2_7.dll and earlier

Breaking Changes:
*   Various interface changes without bumping [`IID_IXAudio2`](crate::xaudio2_0::IID_IXAudio2) in violation of [the COM interface contract rules](https://devblogs.microsoft.com/oldnewthing/20051101-54/?p=33533)
*   ...

Additions:
*   ...



### NuGet Package

Package: [Microsoft.XAudio2.Redist](https://www.nuget.org/packages/Microsoft.XAudio2.Redist/)

Read [Developer guide for redistributable version of XAudio 2.9](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-redistributable) before using for caveats and guidance.

Contains the DLLs: `build/native/{debug,release}/bin/{x86,x64}/xaudio2_9redist.dll`.



### DirectX SDK

Download: [DirectX SDK (June 2010)](https://www.microsoft.com/en-us/download/details.aspx?id=6812)

The DLLs can be found in `C:\Program Files (x86)\Microsoft DirectX SDK (June 2010)\Redist\*.cab`:

| DLL               | 32-bit `*.cab`            | 64-bit `*.cab`            |
| ------------------|:-------------------------:|:-------------------------:|
| `XAudio2_9.dll`   | <span style="opacity: 50%">N/A (comes with Windows 10+)</span> | <span style="opacity: 50%">N/A (comes with Windows 10+)</span> |
| `XAudio2_8.dll`   | <span style="opacity: 50%">N/A (comes with Windows 8+)</span>  | <span style="opacity: 50%">N/A (comes with Windows 8+)</span>  |
| `XAudio2_7.dll`   | `Jun2010_XAudio_x86.cab`  | `Jun2010_XAudio_x64.cab`  |
| `XAudio2_6.dll`   | `Feb2010_XAudio_x86.cab`  | `Feb2010_XAudio_x64.cab`  |
| `XAudio2_5.dll`   | `Aug2009_XAudio_x86.cab`  | `Aug2009_XAudio_x64.cab`  |
| `XAudio2_4.dll`   | `Mar2009_XAudio_x86.cab`  | `Mar2009_XAudio_x64.cab`  |
| `XAudio2_3.dll`   | `Nov2008_XAudio_x86.cab`  | `Nov2008_XAudio_x64.cab`  |
| `XAudio2_2.dll`   | `Aug2008_XAudio_x86.cab`  | `Aug2008_XAudio_x64.cab`  |
| `XAudio2_1.dll`   | `JUN2008_XAudio_x86.cab`  | `JUN2008_XAudio_x64.cab`  |
| `XAudio2_0.dll`   | `Mar2008_XAudio_x86.cab`  | `Mar2008_XAudio_x64.cab`  |



### See Also
*   [XAudio2 Versions](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-versions) (microsoft.com)



[NuGet Package]:                #nuget-package
[DirectX SDK]:                  #directx-sdk
[Microsoft.XAudio2.Redist]:     https://www.nuget.org/packages/Microsoft.XAudio2.Redist/
