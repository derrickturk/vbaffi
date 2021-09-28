use std::{
    marker::PhantomData,
    mem,
    ptr,
};

use winapi::{
    um::{
        oaidl::LPSAFEARRAY,
        oleauto::{
            SafeArrayCreateVector,
            SafeArrayDestroy,
        },
    },
    shared::{
        wtypes::{self, BSTR, VARTYPE},
    },
};

use crate::safeslice::SafeSlice;

pub trait SafeArrayStorable {
    const STORAGE_TYPE: VARTYPE;
}

pub struct SafeVec<T> {
    wrapped: LPSAFEARRAY,
    marker: PhantomData<*const T>,
}

impl<T> SafeVec<T> {
    #[inline]
    pub fn with_capacity(len: usize) -> Option<Self> where T: SafeArrayStorable {
        let arr = unsafe {
            SafeArrayCreateVector(T::STORAGE_TYPE, 0, len as _)
        };
        if arr != ptr::null_mut() {
            Some(Self {
                wrapped: arr,
                marker: PhantomData,
            })
        } else {
            None
        }
    }

    #[inline]
    pub fn new(from: &[T]) -> Option<Self> where T: Clone + SafeArrayStorable {
        let mut this = Self::with_capacity(from.len())?;
        this.as_safeslice().as_mut_slice().clone_from_slice(from);
        Some(this)
    }

    /* yeah, so, obviously we should have split SafeSlice into
     *   SafeSlice and SafeSliceMut, or something...
     */
    #[inline]
    pub fn as_safeslice(&mut self) -> SafeSlice<T> {
        unsafe { SafeSlice::new(self.wrapped).unwrap() }
    }

    /* release ownership and return the raw pointer (e.g. to return via FFI)
     * SAFETY: the caller is now responsible for eventually
     *   calling SafeArrayDestroy!
     */
    #[inline]
    pub unsafe fn into_raw(self) -> LPSAFEARRAY {
        let raw = self.wrapped;
        mem::forget(self);
        raw
    }
}

impl<T> Drop for SafeVec<T> {
    #[inline]
    fn drop(&mut self) {
        eprintln!("dropped");
        unsafe { SafeArrayDestroy(self.wrapped); }
    }
}

macro_rules! storable {
    ($rust_ty:ty, $vt_tag:expr) => {
        impl SafeArrayStorable for $rust_ty {
            const STORAGE_TYPE: VARTYPE = $vt_tag as VARTYPE;
        }
    };
}

storable!(bool, wtypes::VT_BOOL);
storable!(BSTR, wtypes::VT_BSTR);
storable!(i8, wtypes::VT_I1);
storable!(i16, wtypes::VT_I2);
storable!(i32, wtypes::VT_I4);
storable!(i64, wtypes::VT_I8);
storable!(u8, wtypes::VT_UI1);
storable!(u16, wtypes::VT_UI2);
storable!(u32, wtypes::VT_UI4);
storable!(u64, wtypes::VT_UI8);
storable!(f32, wtypes::VT_R4);
storable!(f64, wtypes::VT_R8);
