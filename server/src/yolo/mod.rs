pub mod detection;

use std::path::Path;

use detection::{nms, Detection};
use ndarray::{s, Array, ArrayBase, Dim, OwnedRepr};
use ort::{inputs, CoreMLExecutionProvider, Session, Tensor};
use xcap::image::{imageops::FilterType, DynamicImage, GenericImageView, ImageBuffer, Rgba};

pub struct Model {
    session: ort::Session,
}

const CONFIDENCE_THRESHOLD: f32 = 0.7;
const IOU_THRESHOLD: f32 = 0.5;
const LABELS: [char; 15] = ['n', 'b', 'a', 'k', 'r', 'c', 'p', 'R', 'N', 'A', 'K', 'B', 'C', 'P', '0'];

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Model {
    pub fn new(libs: &Path) -> ort::Result<Self> {
        let model_path = libs.join("model.onnx");
        ort::init().with_execution_providers([CoreMLExecutionProvider::default().build()]).commit()?;

        let session = Session::builder()?.with_model_from_file(model_path)?;

        Ok(Self { session })
    }

    pub fn predict(&self, image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ort::Result<Vec<Detection>> {
        let input = self.process_input(image);
        let outputs = self.session.run(inputs!["images" => input.view()]?)?;
        let detections = self.process_output(outputs["output"].extract_tensor::<f32>()?);
        let detections = nms(detections, IOU_THRESHOLD);
        Ok(detections)
    }

    fn process_input(
        &self,
        image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>> {
        let image = DynamicImage::from(image).resize_exact(640, 640, FilterType::Triangle);
        let mut input = Array::zeros((1, 3, 640, 640));
        for (x, y, pixel) in image.pixels() {
            let [r, g, b, _] = pixel.0;
            input[[0, 0, y as usize, x as usize]] = r as f32 / 255.0;
            input[[0, 1, y as usize, x as usize]] = g as f32 / 255.0;
            input[[0, 2, y as usize, x as usize]] = b as f32 / 255.0;
        }
        input
    }

    fn process_output(&self, outputs: Tensor<f32>) -> Vec<Detection> {
        let output = outputs.view().t().slice(s![.., .., 0]).t().into_owned();
        output
            .rows()
            .into_iter()
            .filter_map(|row| {
                let (class_id, max_prob) = (5..20)
                    .map(|idx| (idx - 5, row[idx]))
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                    .unwrap();

                let conf = row[4] * max_prob;
                if conf < CONFIDENCE_THRESHOLD {
                    None
                } else {
                    Some(Detection::new(row[0], row[1], row[2], row[3], LABELS[class_id], conf))
                }
            })
            .collect()
    }
}
