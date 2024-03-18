use xcap::Window;

pub fn get_windows(title: &str) -> ort::Result<xcap::Window, &str> {
    let windows = Window::all().unwrap();
    for window in windows {
        if window.is_minimized() {
            continue;
        }
        if window.title() != title {
            continue;
        }
        return Ok(window);
    }
    Err("未找到窗口")
}
