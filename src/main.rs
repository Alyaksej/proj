extern crate libc;
use std::os::raw::c_int;
use std::os::unix::io::AsRawFd;
use std::os::unix::net::{UnixDatagram};
use std::io;

extern {
    fn arrayProcessing (ptr_in: *mut c_int, n: c_int) -> *mut c_int;
}

fn main() {
    let socket = UnixDatagram::bind("/tmp/socket.sock");
    let mut data_array: Vec<i32> = Vec::new();
    let mut buffer = [0; 4]; // for byte to i32
    let m = socket.recv(&mut buffer);
    let received_data = i32::from_ne_bytes(buffer);
    data_array.push(received_data);

    // let mut array = [1, 2, 3, 4, 5];
    // let ptr = array.as_mut_ptr();
    // let n = array.len() as c_int;
    //
    // unsafe {
    //     let result = arrayProcessing(ptr, n);
    //     for i in 0..n {
    //         println!("{}", *result.offset(i.try_into().unwrap()));
    //     }
    // }

    let ptr = data_array.as_mut_ptr();
    let n = data_array.len() as c_int;
    if data_array.len() >= 20 {
        unsafe {
            let result = arrayProcessing(ptr, n);
            for i in 0..n {
                println!("{}", *result.offset(i.try_into().unwrap()));
            }
        }
    }
}
