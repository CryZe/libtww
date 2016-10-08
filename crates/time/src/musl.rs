use libtww::std::mem::uninitialized;
use libc::{time_t, c_long, c_int, c_longlong};
use Tm;

fn year_to_secs(year: c_longlong, is_leap: &mut bool) -> c_longlong {
    if year - 2 <= 136 {
        let y = year;
        let mut leaps = (y - 68) >> 2;
        if (y - 68) & 3 == 0 {
            leaps -= 1;
            *is_leap = true;
        } else {
            *is_leap = false;
        }
        return 31536000 * (y - 70) + 86400 * leaps;
    }

    let (mut cycles, centuries, mut leaps, mut rem);

    cycles = (year - 100) / 400;
    rem = (year - 100) % 400;

    if rem < 0 {
        cycles -= 1;
        rem += 400;
    }

    if rem == 0 {
        *is_leap = true;
        centuries = 0;
        leaps = 0;
    } else {
        if rem >= 200 {
            if rem >= 300 {
                centuries = 3;
                rem -= 300;
            } else {
                centuries = 2;
                rem -= 200;
            }
        } else {
            if rem >= 100 {
                centuries = 1;
                rem -= 100;
            } else {
                centuries = 0;
            }
        }
        if rem == 0 {
            *is_leap = false;
            leaps = 0;
        } else {
            leaps = rem / 4;
            rem %= 4;
            *is_leap = rem == 0;
        }
    }

    let is_leap = if *is_leap {
        1
    } else {
        0
    };

    leaps += 97 * cycles + 24 * centuries - is_leap;

    (year - 100) * 31536000 + leaps * 86400 + 946684800 + 86400
}

fn month_to_secs(month: c_int, is_leap: bool) -> c_int {
    const SECS_THROUGH_MONTH: [c_int; 12] = [0,
                                             31 * 86400,
                                             59 * 86400,
                                             90 * 86400,
                                             120 * 86400,
                                             151 * 86400,
                                             181 * 86400,
                                             212 * 86400,
                                             243 * 86400,
                                             273 * 86400,
                                             304 * 86400,
                                             334 * 86400];
    let mut t = SECS_THROUGH_MONTH[month as usize];
    if is_leap && month >= 2 {
        t += 86400;
    }
    t
}

fn tm_to_secs(tm: &Tm) -> c_longlong {
    let mut is_leap = unsafe { uninitialized() };

    let mut year = tm.tm_year;
    let mut month = tm.tm_mon;

    if month >= 12 || month < 0 {
        let mut adj = month / 12;
        month %= 12;
        if month < 0 {
            adj -= 1;
            month += 12;
        }
        year += adj;
    }

    let mut t = year_to_secs(year as c_longlong, &mut is_leap);
    t += month_to_secs(month, is_leap) as c_longlong;
    t += 86400 * (tm.tm_mday - 1) as c_longlong;
    t += 3600 * tm.tm_hour as c_longlong;
    t += 60 * tm.tm_min as c_longlong;
    t += tm.tm_sec as c_longlong;

    t
}

fn secs_to_tm(t: c_longlong, tm: &mut Tm) -> c_int {
    const LEAPOCH: c_longlong = 946684800 + 86400 * (31 + 29);
    const DAYS_PER_400Y: c_longlong = 365 * 400 + 97;
    const DAYS_PER_100Y: c_longlong = 365 * 100 + 24;
    const DAYS_PER_4Y: c_longlong = 365 * 4 + 1;
    const DAYS_IN_MONTH: [c_longlong; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];

    let (mut days, secs, years);
    let (mut remdays, mut remsecs, mut remyears);
    let (mut qc_cycles, mut c_cycles, mut q_cycles);
    let mut months = 0usize;
    let (mut wday, mut yday, leap);

    // Reject time_t values whose year would overflow int
    // if (t < INT_MIN * 31622400LL || t > INT_MAX * 31622400LL)
    // return -1;

    secs = t - LEAPOCH;
    days = secs / 86400;
    remsecs = secs % 86400;
    if remsecs < 0 {
        remsecs += 86400;
        days -= 1;
    }

    wday = (3 + days) % 7;
    if wday < 0 {
        wday += 7;
    }

    qc_cycles = days / DAYS_PER_400Y;
    remdays = days % DAYS_PER_400Y;
    if remdays < 0 {
        remdays += DAYS_PER_400Y;
        qc_cycles -= 1;
    }

    c_cycles = remdays / DAYS_PER_100Y;
    if c_cycles == 4 {
        c_cycles -= 1;
    }
    remdays -= c_cycles * DAYS_PER_100Y;

    q_cycles = remdays / DAYS_PER_4Y;
    if q_cycles == 25 {
        q_cycles -= 1;
    }
    remdays -= q_cycles * DAYS_PER_4Y;

    remyears = remdays / 365;
    if remyears == 4 {
        remyears -= 1;
    }
    remdays -= remyears * 365;

    leap = remyears == 0 && (q_cycles != 0 || c_cycles == 0);
    let leap = if leap {
        1
    } else {
        0
    };
    yday = remdays + 31 + 28 + leap;
    if yday >= 365 + leap {
        yday -= 365 + leap;
    }

    years = remyears + 4 * q_cycles + 100 * c_cycles + 400 * qc_cycles;

    while DAYS_IN_MONTH[months] <= remdays {
        remdays -= DAYS_IN_MONTH[months];
        months += 1;
    }

    // if (years+100 > INT_MAX || years+100 < INT_MIN)
    // return -1;

    tm.tm_year = years as i32 + 100;
    tm.tm_mon = months as i32 + 2;
    if tm.tm_mon >= 12 {
        tm.tm_mon -= 12;
        tm.tm_year += 1;
    }
    tm.tm_mday = remdays as i32 + 1;
    tm.tm_wday = wday as i32;
    tm.tm_yday = yday as i32;

    tm.tm_hour = (remsecs / 3600) as i32;
    tm.tm_min = ((remsecs / 60) % 60) as i32;
    tm.tm_sec = (remsecs % 60) as i32;

    0
}

fn secs_to_zone(_t: c_longlong,
                _local: c_int,
                isdst: &mut c_int,
                offset: &mut c_long,
                oppoff: &mut c_long) {
    // Always use UTC, because the GameCube doesn't
    // handle time zones.
    *isdst = 0;
    *offset = 0;
    *oppoff = 0;
}

pub fn localtime_r<'a>(t: &time_t, tm: &'a mut Tm) -> Result<&'a mut Tm, ()> {
    let mut opp = unsafe { uninitialized() };

    // Reject time_t values whose year would overflow int because
    // __secs_to_zone cannot safely handle them.
    // if (*t < INT_MIN * 31622400LL || *t > INT_MAX * 31622400LL) {
    // 	errno = EOVERFLOW;
    // 	return 0;
    // }

    secs_to_zone(*t as c_longlong,
                 0,
                 &mut tm.tm_isdst,
                 &mut tm.tm_utcoff,
                 &mut opp);

    if secs_to_tm(*t as c_longlong + tm.tm_utcoff as c_longlong, tm) < 0 {
        return Err(());
    }

    Ok(tm)
}

pub fn timegm(tm: &mut Tm) -> time_t {
    let mut new: Tm = unsafe { uninitialized() };
    let t = tm_to_secs(tm);

    if secs_to_tm(t, &mut new) < 0 {
        return -1;
    }

    *tm = new;
    tm.tm_isdst = 0;
    tm.tm_utcoff = 0;

    t as time_t
}

pub fn mktime(tm: &mut Tm) -> time_t {
    let mut new: Tm = unsafe { uninitialized() };
    let mut opp = unsafe { uninitialized() };
    let mut t = tm_to_secs(tm);

    secs_to_zone(t, 1, &mut new.tm_isdst, &mut new.tm_utcoff, &mut opp);

    if tm.tm_isdst >= 0 && new.tm_isdst != tm.tm_isdst {
        t -= opp as c_longlong - new.tm_utcoff as c_longlong;
    }

    t -= new.tm_utcoff as c_longlong;

    // if t as time_t != t {
    //     return -1;
    // }

    secs_to_zone(t, 0, &mut new.tm_isdst, &mut new.tm_utcoff, &mut opp);

    if secs_to_tm(t + new.tm_utcoff as c_longlong, &mut new) < 0 {
        return -1;
    }

    *tm = new;
    t as time_t
}