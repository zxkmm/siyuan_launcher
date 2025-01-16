#![windows_subsystem = "windows"] // as sub-sys, so prevent as console program

use std::fs::read_to_string;
use std::path::Path;
use std::process::Command;
use std::ptr::null_mut;
use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, RegisterClassW};
use winapi::um::winuser::{CS_HREDRAW, CS_VREDRAW, HWND_MESSAGE, WS_DISABLED};
// use winapi::um::winuser::{WNDCLASSW, WM_DESTROY, WM_QUIT}; //TODO way 2
use winapi::um::winuser::{WNDCLASSW}; //TODO lint


fn main() {
    // hided msg window
    unsafe { //TODO
        let class_name = wide_string("SiyuanLauncher");
        let mut wc = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(DefWindowProcW),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: null_mut(),
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            lpszClassName: class_name.as_ptr(),
        };

        if RegisterClassW(&mut wc) == 0 {
            return;
        }

        CreateWindowExW(
            0,
            class_name.as_ptr(),
            wide_string("").as_ptr(),
            WS_DISABLED,
            0,
            0,
            0,
            0,
            HWND_MESSAGE,
            null_mut(),
            null_mut(),
            null_mut(),
        );
    }

    let exe_path = match read_path_file() {
        Ok(path) => path,
        Err(_) => return,
    };

    if !Path::new(&exe_path).exists() {
        return;
    }

    let _ = Command::new(exe_path)
        .spawn()
        .map_err(|_| ())
        .and_then(|_| Ok(()));
}

fn read_path_file() -> Result<String, std::io::Error> {
    let content = read_to_string("path.txt")?;
    Ok(content.trim().to_string())
}

// satisfy win api
fn wide_string(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}