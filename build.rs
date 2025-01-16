#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico"); // 确保在项目根目录放置 icon.ico 文件
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}