extern crate libc;
use std::os::raw::{c_int, c_void};
use std::os::unix::net::{UnixDatagram};
use std::convert::TryInto;
use std::{fs, thread, vec};
use std::time::{Duration, Instant, SystemTime};

// extern {
//     fn arrayProcessing (ptr_in: *mut c_int, n: c_int) -> *mut c_int;
// }

extern {
    fn byteToInt (ptr_in: *mut c_void, len: c_int) -> *mut c_int;
}

const SOCKET_PATH: &str = "/../../../../tmp/socket.sock";
fn main() {
    // Remove socket before start
    if fs::metadata(SOCKET_PATH).is_ok() {
        if let Err(e) = fs::remove_file(SOCKET_PATH) {
            eprintln!("Error removing socket file: {}", e);
            return;
        }
    }

    // Create socket
    let socket = match UnixDatagram::bind(SOCKET_PATH) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error binding socket: {}", e);
            return;
        }
    };

    const MAX_NUMBERS: usize = 20;
    const BUFFER_SIZE: usize = 4 * MAX_NUMBERS;
    const BUFFER_THRESHOLD: usize = BUFFER_SIZE - 10;
    // Buffer for receiving data
    let mut buffer = vec![0; BUFFER_SIZE];
    let lib_ptr = buffer.as_mut_ptr() as *mut c_void;
    let lib_len_max = buffer.len() as c_int;
    let mut buffer_offset: usize = 0;

    let mut all_time = 0;

    let start_time = Instant::now();
    let now = SystemTime::now();
    loop {
        println!("buffer: {:?}", buffer);
        let body_slice: &mut [u8] = &mut buffer[buffer_offset..];
        match socket.recv(body_slice) {
            Ok(len_recv) => {
            println!("len_recv: {:?}", len_recv);
            if len_recv > body_slice.len() {
                println!("Error receiving data: data is to long");
                return;
            };
            buffer_offset += len_recv;
        },
            Err(e) => {
            eprintln!("Error receiving data: {:?}", e);
            return;
        }
        }
        println!("buffer_offset: {:?}", buffer_offset);
        println!("body_slice: {:?}", body_slice);
        unsafe {
            let result = byteToInt(lib_ptr, lib_len_max);
            for i in 0..MAX_NUMBERS {
                println!("result: {}", *result.offset(i.try_into().unwrap()));
            }
        }
        println!("*******************************************************************************");
        all_time += start_time.elapsed().as_micros();
        println!("all_time: {}", all_time);
        if all_time / 1000_000_000 == 5 {
            println!("ololo");
            all_time = 0;
        }

        if buffer_offset >= BUFFER_THRESHOLD {
            buffer.iter_mut().for_each(|x| *x = 0);
            buffer_offset = 0;
        }
        // End of time calculation
        //let duration = start.elapsed();
        //println!("Time elapsed in expensive_function() is: {:?}", duration);
    }
}

fn server_bandwidth(cnt_bytes: usize) {
    let mut string = "ololo";
    println!("string: -------------- {}", cnt_bytes);
}