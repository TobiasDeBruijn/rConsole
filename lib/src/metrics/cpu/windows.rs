use winapi::um::processthreadsapi::GetSystemTimes;
use winapi::shared::minwindef::FILETIME;
use std::time::Duration;
use std::io;

const WINDOWS_SAMPLE_TIME_MILIS: u16 = 500;

fn empty() -> FILETIME {
    FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    }
}

fn filetime_to_u64(f: FILETIME) -> u64 {
    (f.dwHighDateTime as u64) << 32 | (f.dwLowDateTime as u64)
}

fn get_system_times() -> io::Result<(u64, u64)> {
    let mut idle_ticks = empty();
    let mut user_ticks = empty();
    let mut kernel_ticks = empty();

    if unsafe { GetSystemTimes(&mut idle_ticks, &mut kernel_ticks, &mut user_ticks) } == 0 {
        return Err(io::Error::new(io::ErrorKind::Other, "GetSystemTimes() failed"));
    }

    let idle_ticks = filetime_to_u64(idle_ticks);
    let total_ticks = filetime_to_u64(kernel_ticks) + filetime_to_u64(user_ticks);

    Ok((idle_ticks, total_ticks))
}

pub fn get_load_avg() -> io::Result<f64> {
    let (idle_ticks_before, total_ticks_before) = get_system_times()?;
    std::thread::sleep(Duration::from_millis(WINDOWS_SAMPLE_TIME_MILIS as u64));

    let (idle_ticks_after, total_ticks_after) = get_system_times()?;
    let delta_idle_ticks = idle_ticks_after - idle_ticks_before;
    let delta_total_ticks = total_ticks_after - total_ticks_before;

    let diff_ticks = if delta_total_ticks > 0u64 {
        //We need to cast these to f64, otherwise we get integer division and not floating point division
        (delta_idle_ticks as f64) / (delta_total_ticks as f64)
    } else {
        0f64
    };

    Ok(1f64 - diff_ticks)
}

