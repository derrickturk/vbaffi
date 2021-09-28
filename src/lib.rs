use winapi::{
    um::{
        oaidl::LPSAFEARRAY,
    },
    shared::{
        wtypes::BSTR,
    },
};

mod safevec;
use safevec::SafeVec;

#[repr(C)]
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
    if let Some(values) = SafeVec::new(*udts) {
        values.as_slice().iter().map(|udt| sum_values_impl(udt)).sum()
    } else {
        0.0
    }
}

#[no_mangle]
pub unsafe extern "system"
fn alter_values(udt: *const ExampleUDT) {
    if let Some(values) = SafeVec::new((*udt).values) {
        values.as_mut_slice().iter_mut().for_each(|x: &mut f64| *x *= 1.2);
    }
}

#[inline]
fn sum_values_impl(udt: &ExampleUDT) -> f64 {
    if let Some(values) = unsafe { SafeVec::new(udt.values) } {
        values.as_slice().iter().sum()
    } else {
        0.0
    }
}
