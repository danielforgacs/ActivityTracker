use crate::prelude::*;

pub fn secs_to_hours_minutes(secs: SecType) -> (u8, u8) {
    let hours = secs / (60 * 60);
    let minutes = (secs % (60 * 60)) / 60;
    (hours as u8, minutes as u8)
}

pub fn sys_now_secs() -> SecType {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn elapsed_secs(t_start: SecType, t_end: SecType) -> SecType {
    Duration::from_secs(t_end - t_start).as_secs()
}

pub fn elapsed_since(t_start: SecType) -> SecType {
    elapsed_secs(t_start, sys_now_secs())
}
