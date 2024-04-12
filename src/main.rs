extern crate libc;
use std::os::raw::c_int;

extern {
    fn arrayProcessing (ptr_in: *mut c_int, n: c_int) -> *mut c_int;
}

fn main() {
    let mut array = [1, 2, 3, 4, 5];
    let ptr = array.as_mut_ptr();
    let n = array.len() as c_int;

    unsafe {
        let result = arrayProcessing(ptr, n);
        for i in 0..n {
            println!("{}", *result.offset(i.try_into().unwrap()));
        }
    }
}
