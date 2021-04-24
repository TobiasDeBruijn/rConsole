use crate::webserver::AppData;
use crate::endpoints::check_session_id;
use actix_web::{web, post, HttpResponse};
use serde::{Serialize, Deserialize};
use std::io;

#[cfg(windows)]
use winapi::um::processthreadsapi::GetSystemTimes;
#[cfg(windows)]
use winapi::shared::minwindef::FILETIME;
#[cfg(windows)]
use std::time::Duration;

#[cfg(unix)]
use libc::c_int;

#[cfg(windows)]
const WINDOWS_SAMPLE_TIME_MILIS: u64 = 500;

#[derive(Serialize)]
pub struct LoadAverage {
    one:        f64,
    five:       f64,
    fifteen:    f64
}

#[derive(Deserialize)]
pub struct LoadAverageRequest {
    session_id: String
}

#[derive(Serialize)]
pub struct LoadAverageResponse {
    status: i16,
    load:   Option<LoadAverage>
}

#[cfg(unix)]
#[link(name = "c")]
#[allow(dead_code)] //Even though it's used, we still get a warning
extern "C" {
    fn getloadavg(loadavg: *mut f64, nelem: c_int) -> c_int;
}

#[cfg(windows)]
#[inline(always)]
#[allow(dead_code)] //Even though it's used, we still get a warning
fn empty() -> FILETIME {
    FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    }
}

#[cfg(windows)]
#[inline(always)]
#[allow(dead_code)] //Even though it's used, we still get a warning
fn filetime_to_u64(f: FILETIME) -> u64 {
    (f.dwHighDateTime as u64) << 32 | (f.dwLowDateTime as u64)

}

#[cfg(windows)]
#[allow(dead_code)] //Even though it's used, we still get a warning
fn win_poll_load_avg() -> io::Result<f64> {
    let get_system_times = || -> io::Result<(u64, u64)> {
        let mut idle_ticks = empty();
        let mut user_ticks = empty();
        let mut kernel_ticks = empty();

        if unsafe { GetSystemTimes(&mut idle_ticks, &mut kernel_ticks, &mut user_ticks) }== 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "GetSystemTimes() failed"));
        }

        let idle_ticks = filetime_to_u64(idle_ticks);
        let total_ticks = filetime_to_u64(kernel_ticks) + filetime_to_u64(user_ticks);

        Ok((idle_ticks, total_ticks))
    };

    let (idle_ticks_before, total_ticks_before) = get_system_times()?;
    std::thread::sleep(Duration::from_millis(WINDOWS_SAMPLE_TIME_MILIS));

    let (idle_ticks_after, total_ticks_after) = get_system_times()?;
    let delta_idle_ticks = idle_ticks_after - idle_ticks_before;
    let delta_total_ticks = total_ticks_after - total_ticks_before;

    let div_ticks = if delta_total_ticks > 0u64 {
        //We need to cast these to f64, otherwise we get integer division and not floating point division
        (delta_idle_ticks as f64) / (delta_total_ticks as f64)
    } else {
        0f64
    };

    let result = 1f64 - div_ticks;
    Ok(result)
}

fn load_avg() -> io::Result<LoadAverage> {
    #[cfg(windows)]
    {
        let cpu_usage = win_poll_load_avg()?;

        return Ok(LoadAverage {
            one: cpu_usage,
            five: -1f64,    //Unsupported on Windows
            fifteen: -1f64  //Unsupported on Windows
        });
    }

    # [cfg(unix)]
    {
        let mut loads: [f64; 3] = [0.0, 0.0, 0.0];
        if unsafe{ getloadavg(&mut loads[0], 3) } != 3 {
            return Err(io::Error::new(io::ErrorKind::Other, "getloadavg() failed"));
        }

        return Ok(LoadAverage {
            one: loads[0],
            five: loads[1],
            fifteen: loads[2]
        });
    }
}

#[post("/stats/cpu/load")]
pub fn post_get_load_avg(data: web::Data<AppData>, form: web::Form<LoadAverageRequest>) -> HttpResponse {
    let session_valid = check_session_id(&data, &form.session_id);
    if session_valid.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    if !session_valid.unwrap() {
        return HttpResponse::Ok().json(LoadAverageResponse { status: 401, load: None });
    }

    let load = load_avg();
    if load.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().json(LoadAverageResponse {
        status: 200,
        load: Some(load.unwrap())
    })
}