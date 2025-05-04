use serde::Deserialize;
use serde::Serialize;
use xcap::image;
use xcap::image::GenericImage;

#[derive(Serialize, Deserialize, Debug)]
pub struct Window {
    pub id: u32,
    pub title: String,
    pub app_name: String,
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn new(win: &xcap::Window) -> Self {
        let id = win.id().unwrap();
        let title = win.title().unwrap();
        let app_name = win.app_name().unwrap();
        let width = win.width().unwrap();
        let height = win.height().unwrap();
        Self {
            id,
            title,
            app_name,
            width,
            height,
        }
    }
}

#[tauri::command]
pub async fn list_windows() -> Result<Vec<Window>, String> {
    let windows = xcap::Window::all().unwrap();
    if windows.is_empty() {
        return Err("no window".to_string());
    }
    let mut result = vec![];
    for window in windows.iter() {
        result.push(Window::new(window));
    }
    Ok(result)
}

pub struct ListenWindow {
    window: xcap::Window,

    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl ListenWindow {
    #[tracing::instrument]
    pub fn new(target: &Window, w: usize, h: usize) -> Option<Self> {
        let windows = xcap::Window::all().unwrap();
        for window in windows {
            if window.id().unwrap() == target.id {
                return Some(Self {
                    window,
                    x: 0,
                    y: 0,
                    w: 0,
                    h: 0,
                });
            }
        }
        None
    }

    pub fn capture(&self) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        let mut pic = self.window.capture_image().unwrap();
        if self.w > 0 {
            pic = pic.sub_image(self.x, self.y, self.w, self.h).to_image();
        }
        pic
    }

    pub fn set_sub_bound(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
}
