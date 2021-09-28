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
        winerror::S_OK,
    },
};

pub struct SafeVec<T> {
    wrapped: LPSAFEARRAY,
    data: *mut T,
}

impl<T> SafeVec<T> {
    /* SAFETY: ensure that array is a pointer to a valid SAFEARRAY object
     *   (and, ideally, that it contains values of type T - we check the element
     *   size but have no way to check the element type)
     */
    pub unsafe fn new(array: LPSAFEARRAY) -> Option<Self> {
        let mut data: *mut T = ptr::null_mut();
        if SafeArrayAccessData(array, &mut data as *mut _ as *mut _) != S_OK {
            return None;
        }

        if (*array).cDims != 1
          || (*array).cbElements as usize != mem::size_of::<T>() {
              SafeArrayUnaccessData(array);
              return None;
        }

        Some(SafeVec {
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

impl<T> Drop for SafeVec<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { SafeArrayUnaccessData(self.wrapped); }
    }
}
