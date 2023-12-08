use clap::Parser;
use std::{sync::{Arc, Mutex}, process::{Command, self}};

lazy_static::lazy_static! {
    static ref MONITORS: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

#[derive(Parser, Debug)]
#[command(author = "Flora Hill", version = "0.0.1", about = "A simple tool to execute a command when a number of monitors is reached", long_about = None)]
struct Args {
    #[arg(short = 'r', long = None, default_value = "1")]
    rate: usize,
    #[arg(short = 'i', long = None, default_value = "false")]
    ignore_equilibrium: bool,
    #[arg(short = 't', long = None)]
    target: usize,
    #[arg(short = 'c', long = None)]
    command: String,
}

fn find_monitors() {
    *MONITORS.lock().unwrap() = 0;

    unsafe {
        unsafe extern "system" fn handler(
            _hmonitor: windows::Win32::Graphics::Gdi::HMONITOR,
            _hdc: windows::Win32::Graphics::Gdi::HDC,
            _rect: *mut windows::Win32::Foundation::RECT,
            _lparam: windows::Win32::Foundation::LPARAM,
        ) -> windows::Win32::Foundation::BOOL {
            *MONITORS.lock().unwrap() += 1;
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

fn main() {
    let args = Args::parse();
    let rate = args.rate;
    let target = args.target;
    let command = args.command.split(" ").collect::<Vec<&str>>();

    let mut equilibrium = get_count() == target;

    loop {
        let current = get_count();
        if current == target {
            if !equilibrium || args.ignore_equilibrium {
                Command::new(&command[0])
                    .args(&command[1..])
                    .spawn()
                    .expect("Failed to execute command");
                
                process::exit(0);
            }
        } else {
            equilibrium = false;
        }

        std::thread::sleep(std::time::Duration::from_secs(rate as u64));
    }
}

fn get_count() -> usize {
    find_monitors();
    *MONITORS.lock().unwrap()
}
