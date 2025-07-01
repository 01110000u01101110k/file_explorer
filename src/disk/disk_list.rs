use windows::{
    Win32::Storage::FileSystem::GetLogicalDrives,
};
use std::env::consts::OS;

pub fn get_disk_list() -> Vec<char> {
    let mut disk_list: Vec<char> = Vec::new();

    match OS {
        "windows" => {
            let drives_mask = unsafe { GetLogicalDrives() };

            for i in 0..26 { // todo: розібрати потім детальніше тему масок
                if drives_mask & (1 << i) != 0 {
                    let letter = (b'A' + i as u8) as char;

                    disk_list.push(letter);
                }
            }
        },
        "linux" => {
            // todo - реалізувати версію для лінукс систем
            /*let file = File::open("/proc/mounts")?;
            let reader = BufReader::new(file);
            let mut mount_points = Vec::new();

            for line in reader.lines() {
                if let Ok(line) = line {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    
                    if fields.len() >= 2 {
                        mount_points.push(fields[1].to_string());
                    }
                }
            }*/
        },
        _ => {
            println!("операційна система не підтримується");
        }
    }

    disk_list
}