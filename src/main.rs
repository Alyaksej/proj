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
    //let mut buffer = vec![0u8, BUFFER_SIZE as u8];

    // loop {
    //     let mut buffer = [0u8; 4]; // for byte to i32
    //     if let Err(e) = socket.recv_from(&mut buffer) {
    //         eprintln!("Error receiving data: {}", e);
    //         continue;
    //     };
    //     let received_data = i32::from_be_bytes(buffer);
    //     data_array.push(received_data);
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

    const MAX_NUMBERS: usize = 5;
    const BUFFER_SIZE: usize = 4 * MAX_NUMBERS;

    loop {
        let mut buffer = vec![0u8, (BUFFER_SIZE as i32).try_into().unwrap()];

        if let Err(e) = socket.recv_from(&mut buffer) {
            eprintln!("Error receiving data: {}", e);
            continue;
        }
        println!("buffer{}",buffer[0]);
        let num_received_bytes = buffer.len();
        let num_received_numbers = num_received_bytes / std::mem::size_of::<i32>();

        for n in 0..num_received_numbers {
            let start_index = n * std::mem::size_of::<i32>();
            let end_index = (n + 1) * std::mem::size_of::<i32>();
            let number_bytes = &buffer[start_index..end_index];

            let received_data = i32::from_be_bytes(number_bytes.try_into().expect("Invalid byte sequence"));
            data_array.push(received_data);
        }

        let ptr = data_array.as_mut_ptr();
        let n = data_array.len() as c_int;

        if data_array.len() >= 5 {
            unsafe {
                let result = arrayProcessing(ptr, n);
                for i in 0..n {
                    println!("{}", *result.offset(i.try_into().unwrap()));
                }
            }

            // Clearing of processed data
            data_array.drain(0..num_received_numbers);

            if let Err(e) = fs::remove_file(SOCKET_PATH) {
                eprintln!("Error removing socket file: {}", e);
            }
        }
    }
}