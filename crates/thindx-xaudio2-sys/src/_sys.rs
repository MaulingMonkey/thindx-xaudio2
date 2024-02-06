#![doc = include_str!("../Readme.md")]
#![cfg(windows)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[macro_use] mod macros;
#[path = "../doc/_doc.rs"] pub mod _doc;
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

/// Allows for the use of an interface type as the first element of a `#[repr(C)]` struct when implementing an interface without resorting to transmutes.
#[doc(hidden)] pub unsafe trait FromVtable {
    /// The underlying vtable type
    type Vtable;

    /// Allow the use of this interface type as e.g. the first field of a struct to simulate inheriting it.
    ///
    /// ### Safety
    /// Most code will rely on interfaces having a "valid" vtable for soundness.
    /// What constitutes "valid" is hideously underdefined and interface specific.
    /// The concept of "valid" also extends to the fns present and their implementations behaving "correctly".
    /// By using this method, you assert that the vtable is indeed "valid" and the fns within are "correct".
    unsafe fn from_vtable(vtable: &'static Self::Vtable) -> Self;

    /// Like [Self::from_vtable], but even more unsafe.
    ///
    /// ### Safety
    /// All the safety requirements of [FromVtable::from_vtable] apply.  Additionally, it's almost certainly unsound to:
    /// *   pass a `null()` vtable
    /// *   pass a non-`'static` vtable
    /// *   pass a dangling vtable
    /// *   yank the vtable or the fns it references out from underneath the interface by e.g. unloading the DLL
    unsafe fn from_vtable_unbounded(vtable: *const Self::Vtable) -> Self;
}
