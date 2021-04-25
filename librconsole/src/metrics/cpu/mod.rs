use std::io;

//Modules marked with a #[cfg(OS)] are always reported unused, hence the #[allow(dead_code)]
//See: https://github.com/rust-lang/cargo/issues/9414

//#[allow(dead_code)]
#[cfg(windows)]
mod windows;

//#[allow(dead_code)]
#[cfg(unix)]
mod unix;

pub fn get_load_avg() -> io::Result<f64> {
    #[cfg(windows)]
    {
        return windows::get_load_avg();
    }

    #[cfg(unix)]
    {
        return Ok(unix::get_load_avg()?.0);
    }
}