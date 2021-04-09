use std::time::Duration;
use std::thread::sleep;

pub struct Monitor;

impl Monitor {
    #[cfg(target_os = "macos")]
    pub fn display(_image: &str) {}

    #[cfg(target_os = "macos")]
    pub fn m_sleep() {
        sleep(Duration::from_secs(10));
    }

    #[cfg(target_os = "linux")]
    pub fn m_sleep() {
        sleep(Duration::from_secs(60 * 30));
    }

    #[cfg(target_os = "linux")]
    pub fn display(image: &str) {
        match std::process::Command::new("sudo")
            .arg("epaper")
            .arg(image)
            .status() {
            Ok(_status) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

}

