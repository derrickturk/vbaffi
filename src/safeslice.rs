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

use std::{mem, ptr, slice};

use winapi::{
    um::{
        oaidl::LPSAFEARRAY,
        oleauto::{
            SafeArrayAccessData,
            SafeArrayUnaccessData,
        },
    },
    shared::{
        winerror::{
            E_INVALIDARG,
            E_UNEXPECTED,
            S_OK,
        },
    },
};

#[derive(Debug)]
pub struct SafeSlice<T> {
    wrapped: LPSAFEARRAY,
    data: *mut T,
}

#[derive(Copy, Clone, Debug)]
pub enum SafeSliceError {
    InvalidSafeArray,
    LockFailed,
    InvalidDimensions(usize),
    InvalidElementSize(usize),
}

impl<T> SafeSlice<T> {
    /* SAFETY: ensure that array is a pointer to a valid SAFEARRAY object
     *   (and, ideally, that it contains values of type T - we check the element
     *   size but have no way to check the element type)
     */
    pub fn new(array: LPSAFEARRAY) -> Result<Self, SafeSliceError> {
        let mut data: *mut T = ptr::null_mut();
        let res = unsafe {
            SafeArrayAccessData(array, &mut data as *mut _ as *mut _)
        };
        match res {
            E_INVALIDARG => return Err(SafeSliceError::InvalidSafeArray),
            E_UNEXPECTED => return Err(SafeSliceError::LockFailed),
            S_OK => {},
            _ => panic!("undocumented HRESULT from SafeArrayAccessData!"),
        };

        let dims = unsafe { (*array).cDims as usize };
        let elt_size = unsafe { (*array).cbElements as usize };

        if dims != 1 {
              unsafe { SafeArrayUnaccessData(array); }
              return Err(SafeSliceError::InvalidDimensions(dims));
        }

        if elt_size != mem::size_of::<T>() {
              unsafe { SafeArrayUnaccessData(array); }
              return Err(SafeSliceError::InvalidElementSize(elt_size));
        }

        Ok(SafeSlice {
            wrapped: array,
            data: data,
        })
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.data,
              (*self.wrapped).rgsabound[0].cElements as usize)
        }
    }

    #[inline]
    pub fn as_mut_slice(&self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.data,
              (*self.wrapped).rgsabound[0].cElements as usize)
        }
    }
}

impl<T> Drop for SafeSlice<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { SafeArrayUnaccessData(self.wrapped); }
    }
}
