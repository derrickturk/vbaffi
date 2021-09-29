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

use std::ptr;

use winapi::{
    um::{
        oaidl::LPSAFEARRAY,
    },
    shared::{
        wtypes::BSTR,
    },
};

mod safeslice;
use safeslice::SafeSlice;

mod safevec;
use safevec::SafeVec;

#[repr(C, packed(4))]
pub struct ExampleUDT {
    magic: i32,
    name: BSTR,
    values: LPSAFEARRAY,
}

#[no_mangle]
pub unsafe extern "system"
fn sum_values(udt: *const ExampleUDT) -> f64 {
    sum_values_impl(&*udt)
}

#[no_mangle]
pub unsafe extern "system"
fn hypersum_values(udts: *mut LPSAFEARRAY) -> f64 {
    if let Ok(values) = SafeSlice::new(*udts) {
        values.as_slice().iter().map(|udt| sum_values_impl(udt)).sum()
    } else {
        0.0
    }
}

#[no_mangle]
pub unsafe extern "system"
fn alter_values(udt: *const ExampleUDT) {
    if let Ok(values) = SafeSlice::new((*udt).values) {
        values.as_mut_slice().iter_mut().for_each(|x: &mut f64| *x *= 1.2);
    }
}

#[no_mangle]
pub unsafe extern "system"
fn make_array() -> LPSAFEARRAY {
    SafeVec::new(&[1i32, 2, 3, 4])
      .map(|v| v.into_raw())
      .unwrap_or(ptr::null_mut())
}

#[inline]
fn sum_values_impl(udt: &ExampleUDT) -> f64 {
    if let Ok(values) = SafeSlice::new(udt.values) {
        values.as_slice().iter().sum()
    } else {
        0.0
    }
}
