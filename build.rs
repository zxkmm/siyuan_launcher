#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico"); // icon.ico from siyuan official repo 
    res.compile().unwrap();
}

#[cfg(not(windows))]

fn main() {}
