/*
Copyright (c) 2021 Derrick W. Turk | terminus data science, LLC

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

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
