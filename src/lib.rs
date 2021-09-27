use std::{ptr, slice};

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
        wtypes::BSTR,
    },
};

mod guard;
use guard::Guard;

#[repr(C)]
pub struct ExampleUDT {
    magic: i32,
    name: BSTR,
    values: LPSAFEARRAY,
}

#[no_mangle]
pub unsafe extern "system"
fn sum_values(udt: *const ExampleUDT) -> f64 {
    let values = (*udt).values;
    let mut val_buf: *const f64 = ptr::null();

    if SafeArrayAccessData(values, &mut val_buf as *mut _ as *mut _) != S_OK {
        return 0.0;
    }

    let _guard = Guard::new(|| { SafeArrayUnaccessData(values); });

    if (*values).cDims != 1 {
        return 0.0;
    }

    if (*values).cbElements != 8  {
        return 0.0;
    }

    let n = (*values).rgsabound[0].cElements as usize;
    let values = slice::from_raw_parts(val_buf, n);

    values.iter().sum()
}
