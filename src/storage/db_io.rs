use crate::prelude::*;

pub fn read(path: &path::PathBuf) -> Vec<Activity> {
    let mut file_handle = std::fs::File::open(path).unwrap();
    let mut buf = String::new();
    file_handle.read_to_string(&mut buf).unwrap();
    let activity_serial: Vec<ActivitySerial> =
        serde_json::from_str(buf.as_str()).unwrap();
    let data: Vec<Activity> =
        activity_serial.into_iter().map(Activity::from).collect();
    data
}

pub fn read_as_serialised(path: &path::PathBuf) -> Vec<ActivitySerial> {
    read(path)
        .into_iter()
        .map(ActivitySerial::from)
        .collect::<Vec<ActivitySerial>>()
}

pub fn write(path: &path::PathBuf, data: Vec<Activity>) {
    let activity_serials: Vec<ActivitySerial> =
        data.into_iter().map(ActivitySerial::from).collect();
    let data_serialised =
        serde_json::to_string_pretty(&activity_serials).unwrap();
    let mut file_handle = std::fs::File::create(path).unwrap();
    file_handle
        .write_all(data_serialised.as_bytes())
        .expect("CAN NOT WRITE ALL.");
}
