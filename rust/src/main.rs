use std::io::{self, Read};

// Experimental prototype (WIP).
#[allow(dead_code)]
fn read_i32() -> io::Result<i32> {
    let mut stdin = io::stdin();
    let mut buffer = [0u8; 1];
    let mut number = 0;
    let whitespace = [b' ', b'\n', b'\t'];

    // Skip leading whitespace
    stdin.read_exact(&mut buffer)?;
    while whitespace.contains(&buffer[0]) {
        stdin.read_exact(&mut buffer)?;
    }

    // Determine the sign
    let sign = if buffer[0] == b'-' {
        stdin.read_exact(&mut buffer)?;
        -1
    } else {
        1
    };

    // Read and process digits
    while b'0' <= buffer[0] && buffer[0] <= b'9' {
        let digit = (buffer[0] - b'0') as i32;
        number = number * 10 + digit;
        stdin.read_exact(&mut buffer)?;
    }
    number *= sign;
    Ok(number)
}

fn main() {}
