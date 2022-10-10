macro_rules! interfaces {
    ($(
        $(#[doc = $interface_doc:literal])*
        $(#[iid = $iid:tt])?
        $interface_vis:vis interface $interface:ident ( $interface_vtable:ident ) $( => unsafe $base:ident ( $base_vtable:ident ) )? {
            $(
                $(#[doc = $fn_doc:literal])*
                pub unsafe fn $method:ident ( &self $(, $param_id:ident : $param_ty:ty )* $(,)? ) -> $return_ty:ty;
            )*
        }
    )*) => {$(
        $(#[doc = $interface_doc])*
        #[repr(C)] $interface_vis struct $interface ( * const $interface_vtable );
        #[doc(hidden)] #[repr(C)] $interface_vis struct $interface_vtable {
            $(
                pub base: $base_vtable,
            )?
            $(
                $(#[doc = $fn_doc])*
                pub $method : unsafe extern "system" fn(This: *const $interface $(, $param_id : $param_ty)*) -> $return_ty,
            )*
        }
        impl $interface {
            $(
                $(#[doc = $fn_doc])*
                #[allow(dead_code)] pub unsafe fn $method(&self $(, $param_id : $param_ty)*) -> $return_ty {
                    unsafe { ((*self.0).$method)(self $(, $param_id)*) }
                }
            )*
        }
        unsafe impl crate::FromVtable for $interface {
            type Vtable = $interface_vtable;
            unsafe fn from_vtable(vtable: &'static $interface_vtable) -> Self { Self(vtable) }
            unsafe fn from_vtable_unbounded(vtable: *const $interface_vtable) -> Self { Self(vtable) }
        }
        $(
            impl core::ops::Deref for $interface {
                type Target = $base;
                fn deref(&self) -> &Self::Target { unsafe { core::mem::transmute(self) } }
            }
        )?
        interfaces!(@iid $interface $($iid)?);
    )*};

    (@iid $interface:ident           ) => {};
    (@iid $interface:ident None      ) => { unsafe impl mcom::AsIUnknown for $interface { fn as_iunknown(&self) -> &IUnknown { self } } };
    (@iid $interface:ident $iid:expr ) => { impl winapi::Interface for $interface { fn uuidof() -> winapi::shared::guiddef::GUID { $iid.into() } } };
}
