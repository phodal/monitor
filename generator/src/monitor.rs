use std::time::Duration;
use std::thread::sleep;

pub struct Monitor;

impl Monitor {
    #[cfg(target_os = "macos")]
    pub fn display() {}

    #[cfg(target_os = "macos")]
    pub fn m_sleep() {
        sleep(Duration::from_secs(10));
    }

    #[cfg(target_os = "linux")]
    pub fn m_sleep() {
        sleep(Duration::from_secs(60 * 30));
    }

    #[cfg(target_os = "linux")]
    pub fn display() {
        match std::process::Command::new("sudo")
            .arg("epaper")
            .arg("monitor.bmp")
            .status() {
            Ok(_status) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

}

