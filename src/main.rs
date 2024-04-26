extern crate libc;
use std::os::raw::c_int;
use std::os::unix::net::{UnixDatagram};
use std::convert::TryInto;
use std::fs;

extern {
    fn arrayProcessing (ptr_in: *mut c_int, n: c_int) -> *mut c_int;
}
const SOCKET_PATH: &str = "/../../../../tmp/socket.sock";
fn main() {
    //  Create unix_socket
    let socket = match UnixDatagram::bind(SOCKET_PATH) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error binding socket: {}", e);
            return;
        }
    };
    let mut data_array: Vec<i32> = Vec::new();

    loop {
        let mut buffer = [0u8; 4]; // for byte to i32
        if let Err(e) = socket.recv_from(&mut buffer) {
            eprintln!("Error receiving data: {}", e);
            continue;
        };
        let received_data = i32::from_be_bytes(buffer);
        data_array.push(received_data);

        let ptr = data_array.as_mut_ptr();
        let n = data_array.len() as c_int;
        if data_array.len() >= 5 {
            unsafe {
                let result = arrayProcessing(ptr, n);
                for i in 0..n {
                    println!("{}", *result.offset(i.try_into().unwrap()));
                }
            }
            data_array.clear();
        }
        if let Err(e) = fs::remove_file(SOCKET_PATH) {
            eprintln!("Error removing socket file: {}", e);
        }
    }
}