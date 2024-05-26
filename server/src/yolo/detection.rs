#[derive(Debug, Clone, Copy)]
pub struct Detection {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub confidence: f32,
    pub label: char,
}

impl Detection {
    pub fn new(x: f32, y: f32, w: f32, h: f32, label: char, confidence: f32) -> Self {
        Self {
            x0: x - w / 2.0,
            x1: x + w / 2.0,
            y0: y - h / 2.0,
            y1: y + h / 2.0,
            x,
            y,
            w,
            h,
            label,
            confidence,
        }
    }

    // 计算两个检测框的IOU(交并比)
    pub fn iou(&self, other: &Detection) -> f32 {
        let self_area = (self.x1 - self.x0) * (self.y1 - self.y0);
        let other_area = (other.x1 - other.x0) * (other.y1 - other.y0);
        let intersection_x = (self.x1.min(other.x1) - self.x0.max(other.x0)).max(0.0);
        let intersection_y = (self.y1.min(other.y1) - self.y0.max(other.y0)).max(0.0);
        let intersection_area = intersection_x * intersection_y;

        intersection_area / (self_area + other_area - intersection_area)
    }
}

// 使用IOU计算去除重叠的检测框（非极大值抑制）
pub fn nms(mut detections: Vec<Detection>, threshold: f32) -> Vec<Detection> {
    // 置信度排序
    detections.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

    let mut filtered_detections = Vec::new();

    // IOU剔除
    while !detections.is_empty() {
        let current_detection = detections.remove(0);
        filtered_detections.push(current_detection);
        detections.retain(|detection| current_detection.iou(detection) < threshold);
    }
    filtered_detections
}
