use libc::c_int;
use std::io;

#[link(name = "c")]
extern "C" {
    fn getloadavg(loadavg: *mut f64, nelem: c_int) -> c_int;
}

pub fn get_load_avg() -> io::Result<(f64, f64, f64)> {
    let mut loads: [f64; 3] = [0f64, 0f64, 0f64];
    if unsafe{ getloadavg(&mut loads[0], 3) } != 3 {
        return Err(io::Error::new(io::ErrorKind::Other, "getloadavg() failed"));
    }

    return Ok((loads[0], loads[1], loads[2]))
}