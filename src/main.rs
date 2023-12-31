use lazy_static::lazy_static;
use std::{
    process::Command,
    sync::{Arc, Mutex},
};
use types::AppConfig;

mod types;

fn main() {
    find_monitors();
    let args: Vec<String> = std::env::args().collect();
    let persist = !args.contains(&String::from("-p"));

    println!("Args: {:?}", args);

    // Setup tempdir
    if std::path::Path::new(&*TEMP_DIR).exists() && persist {
        std::fs::remove_dir_all(&*TEMP_DIR).unwrap();
    }

    if !std::path::Path::new(&*TEMP_DIR).exists() {
        std::fs::create_dir_all(&*TEMP_DIR).unwrap();
    }

    // If arg -i <location> is passed, identify monitors
    if args.len() > 2 && args[1] == "-i" {
        identify(args[2].clone());
        return;
    }

    // If arg -u <url> is passed, get config from url
    if args.len() > 2 && args[1] == "-u" {
        let config = reqwest::blocking::get(&args[2])
            .unwrap()
            .json::<AppConfig>()
            .unwrap();
        println!("Got config from url");
        spawn_chrome(config);
        return;
    }

    let file = std::fs::read_to_string("config.json").unwrap();
    let config: AppConfig = serde_json::from_str(&file).unwrap();
    spawn_chrome(config);
}

#[derive(Debug)]
struct Monitor {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
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

fn find_monitors() {
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
                width: (*rect).right - (*rect).left,
                height: (*rect).bottom - (*rect).top,
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
}

fn make_for(id: String) -> String {
    let folder = format!("{}\\{}", &*TEMP_DIR, id);
    if std::path::Path::new(&folder).exists() {
        return folder;
    }

    std::fs::create_dir_all(&folder).unwrap();
    folder
}

fn identify(location: String) {
    let mon = MONITORS.lock().unwrap();
    for (index, mon) in mon.iter().enumerate() {
        println!("Launching chrome on monitor #{index}");
        Command::new(location.clone())
            .arg(format!("--app=data:text/html,<html><body style=\"margin:0;padding:0;display:grid;place-items:center;\"><h1 style=\"font-size:7vmax;font-family:sans-serif;\">Screen: {}</h1></body></html>", index))
            .arg(format!("--window-position={},{}", mon.left, mon.top))
            .arg(format!("--user-data-dir={}", make_for(index.to_string())))
            .arg("--kiosk")
            .spawn()
            .unwrap();
    }
}

fn spawn_chrome(config: AppConfig) {
    let monitor = MONITORS.lock().unwrap();
    let args: Vec<String> = std::env::args().collect();
    println!("Found {} monitor(s)", monitor.len());

    for (index, conf) in config.windows.iter().enumerate() {
        println!("Launching chrome on monitor #{}", conf.screen);

        let mon = monitor
            .get(conf.screen as usize)
            .expect(format!("No monitor found for screen #{}", conf.screen).as_str());

        let url = conf.url.clone();
        if url.contains(",") {
            let urls = url.split(",").collect::<Vec<&str>>();
            for (i, url) in urls.iter().enumerate() {
                let height_slice = mon.height / urls.len() as i32;
                Command::new(config.chrome_path.clone())
                    .arg("--allow-running-insecure-content")
                    .arg(format!("--app={}", url))
                    .arg(format!(
                        "--window-position={},{}",
                        mon.left,
                        mon.top + (height_slice * i as i32)
                    ))
                    .arg(format!("--window-size={},{}", mon.width, height_slice))
                    .arg(format!(
                        "--user-data-dir={}",
                        make_for(format!("{index}-{i}"))
                    ))
                    .spawn()
                    .unwrap();
            }

            continue;
        }

        Command::new(config.chrome_path.clone())
            .arg("--allow-running-insecure-content")
            .arg(format!("--app={}", conf.url))
            .arg(format!("--window-position={},{}", mon.left, mon.top))
            .arg(format!("--user-data-dir={}", make_for(index.to_string())))
            .arg({
                if args.contains(&String::from("-f")) {
                    "--start-fullscreen"
                } else {
                    "--kiosk"
                }
            })
            .spawn()
            .unwrap();
    }
}
