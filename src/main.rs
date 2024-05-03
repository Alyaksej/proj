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
    // Receive on number
    // Create unix_socket
    if fs::metadata(SOCKET_PATH).is_ok() {
        if let Err(e) = fs::remove_file(SOCKET_PATH) {
            eprintln!("Error removing socket file: {}", e);
            return;
        }
    }
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
    }

    // Receive array of numbers
    // let socket = match UnixDatagram::bind(SOCKET_PATH) {
    //     Ok(s) => s,
    //     Err(e) => {
    //         eprintln!("error binding socket: {}", e);
    //         return;
    //     }
    // };
    // let mut data_array: Vec<i32> = Vec::new();
    //
    // const MAX_NUMBERS: usize = 5;
    // const BUFFER_SIZE: usize = 4 * MAX_NUMBERS;
    // loop {
    //     let mut buffer = vec![0; BUFFER_SIZE];
    //     let _ = socket.recv(&mut buffer);
    //
    //     println!("buffer{:?}", buffer);
    //
    //     for i in (0..BUFFER_SIZE).step_by(4) {
    //         let bytes: [u8; 4] = buffer[i..(i + 4)].try_into().unwrap();
    //         let number = i32::from_ne_bytes(bytes);
    //         data_array.push(number);
    //         println!("data_array: {:?}", data_array);
    //     }
    //     let ptr = data_array.as_mut_ptr();
    //     let n = data_array.len() as c_int;
    //     println!("step1");
    //     unsafe {
    //         println!("step2");
    //         let result = arrayProcessing(ptr, n);
    //         for i in 0..n {
    //             println!("step3");
    //             println!("result: {}", *result.offset(i.try_into().unwrap()));
    //         }
    //     }
    //     data_array.clear();
    //     if let Err(e) = fs::remove_file(SOCKET_PATH) {
    //         eprintln!("Error removing socket file: {}", e);
    //     }
    // }

    // Receive one number to array
    //  Create unix_socket
    // let socket = match UnixDatagram::bind(SOCKET_PATH) {
    //     Ok(s) => s,
    //     Err(e) => {
    //         eprintln!("error binding socket: {}", e);
    //         return;
    //     }
    // };
    //
    // const MAX_NUMBERS: usize = 5;
    // const BUFFER_SIZE: usize = 4 * MAX_NUMBERS;
    // let mut data_array: Vec<i32> = Vec::new();
    //
    // loop {
    //     let mut buffer = vec![0; BUFFER_SIZE];
    //     for _i in 0.. BUFFER_SIZE {
    //         if let Err(e) = socket.recv_from(&mut buffer) {
    //             eprintln!("Error receiving data: {}", e);
    //             continue;
    //         };
    //         println!("buffer: {:?}", buffer);
    //     }
    //
    //     let ptr = data_array.as_mut_ptr();
    //     let n = data_array.len() as c_int;
    //     if data_array.len() >= 5 {
    //         unsafe {
    //             let result = arrayProcessing(ptr, n);
    //             for i in 0..n {
    //                 println!("{}", *result.offset(i.try_into().unwrap()));
    //             }
    //         }
    //         data_array.clear();
    //     }
    //     if let Err(e) = fs::remove_file(SOCKET_PATH) {
    //         eprintln!("Error removing socket file: {}", e);
    //     }
    // }
}