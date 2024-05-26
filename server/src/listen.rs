use tracing::debug;
use xcap::image::{self, GenericImage};

pub struct ListenWindow {
    window: xcap::Window,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

unsafe impl Send for ListenWindow {}
unsafe impl Sync for ListenWindow {}

impl ListenWindow {
    #[tracing::instrument]
    pub fn new(title: String) -> Option<Self> {
        let windows = xcap::Window::all().unwrap();
        for window in windows {
            debug!("match window {}", window.title());
            if window.is_maximized() || window.title() != title {
                continue;
            };
            return Some(Self { window, x: 0, y: 0, w: 0, h: 0 });
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

    pub fn set(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }
}
