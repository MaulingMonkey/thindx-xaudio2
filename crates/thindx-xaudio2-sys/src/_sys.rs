//! \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/xaudio2/xaudio2-apis-portal)\]

#![cfg(windows)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[path = "sys20.rs"] pub mod xaudio2_0;
#[path = "sys21.rs"] pub mod xaudio2_1;
#[path = "sys22.rs"] pub mod xaudio2_2;
#[path = "sys23.rs"] pub mod xaudio2_3;
#[path = "sys24.rs"] pub mod xaudio2_4;
#[path = "sys25.rs"] pub mod xaudio2_5;
#[path = "sys26.rs"] pub mod xaudio2_6;
#[path = "sys27.rs"] pub mod xaudio2_7;
#[path = "sys28.rs"] pub mod xaudio2_8;
#[path = "sys29.rs"] pub mod xaudio2_9;

const fn guid(data1: u32, data2: u16, data3: u16, data4: u64) -> winapi::shared::guiddef::GUID {
    winapi::shared::guiddef::GUID { Data1: data1, Data2: data2, Data3: data3, Data4: data4.to_be_bytes() }
}
