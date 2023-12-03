use lazy_static::lazy_static;
use std::{
    process::Command,
    sync::{Arc, Mutex},
};
use types::AppConfig;

mod types;

fn main() {
    let file = std::fs::read_to_string("config.json").unwrap();
    let config: AppConfig = serde_json::from_str(&file).unwrap();
    spawn_chrome(config);
}

#[derive(Debug)]
struct Monitor {
    left: i32,
    top: i32,
}

lazy_static! {
    static ref MONITORS: Arc<Mutex<Vec<Monitor>>> = Arc::new(Mutex::new(Vec::new()));
    static ref TEMP_DIR: String = format!(
        "{}\\rm_winmove",
        std::env::temp_dir().to_str().unwrap().to_string()
    );
}

fn collect(mon: Monitor) {
    let mons = MONITORS.clone();
    let mut mons = mons.lock().unwrap();
    mons.push(mon);
}

fn spawn_chrome(config: AppConfig) {
    if std::path::Path::new(&*TEMP_DIR).exists() {
        std::fs::remove_dir_all(&*TEMP_DIR).unwrap();
    }

    std::fs::create_dir_all(&*TEMP_DIR).unwrap();

    unsafe {
        unsafe extern "system" fn handler(
            _hmonitor: windows::Win32::Graphics::Gdi::HMONITOR,
            _hdc: windows::Win32::Graphics::Gdi::HDC,
            rect: *mut windows::Win32::Foundation::RECT,
            _lparam: windows::Win32::Foundation::LPARAM,
        ) -> windows::Win32::Foundation::BOOL {
            collect(Monitor {
                left: (*rect).left,
                top: (*rect).top,
            });

            windows::Win32::Foundation::BOOL::from(true)
        }

        let proc = windows::Win32::Graphics::Gdi::MONITORENUMPROC::Some(handler);

        windows::Win32::Graphics::Gdi::EnumDisplayMonitors(
            None,
            None,
            proc,
            windows::Win32::Foundation::LPARAM(0),
        );
    }

    let monitor = MONITORS.lock().unwrap();
    println!("Found {} monitor(s)", monitor.len());

    for (index, conf) in config.windows.iter().enumerate() {
        println!("Launching chrome on monitor #{index}");

        let mon = monitor
            .get(conf.screen as usize)
            .expect(format!("No monitor found for screen #{}", conf.screen).as_str());
        let folder = format!("{}\\{}", &*TEMP_DIR, mon.left);
        std::fs::create_dir_all(&folder).unwrap();

        Command::new(config.chrome_path.clone())
            .arg(format!("--app={}", conf.url))
            .arg(format!("--window-position={},{}", mon.left, mon.top))
            .arg(format!("--user-data-dir={}", folder))
            .arg("--kiosk")
            .spawn()
            .unwrap();
    }
}
