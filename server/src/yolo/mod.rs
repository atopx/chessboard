pub mod detection;

use std::path::PathBuf;

use detection::{nms, Detection};
use ndarray::{s, Array, ArrayBase, Dim, OwnedRepr};
use ort::{inputs, CoreMLExecutionProvider, Session, Tensor};
use xcap::image::{
    imageops::FilterType, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba,
};

pub struct Model {
    session: Session,
}

const CONFIDENCE_THRESHOLD: f32 = 0.7f32;
const IOU_THRESHOLD: f32 = 0.5f32;
const LABELS: [char; 15] = [
    'n', 'b', 'a', 'k', 'r', 'c', 'p', 'R', 'N', 'A', 'K', 'B', 'C', 'P', '0',
];

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Model {
    pub fn new(model_path: PathBuf) -> ort::Result<Self> {
        ort::init()
            .with_execution_providers([CoreMLExecutionProvider::default().build()])
            .commit()?;

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
        let image = DynamicImage::from(image);
        let new_img = image.resize_exact(640, 640, FilterType::Triangle);
        // let new_img = image.resize_exact(640, 640, FilterType::Lanczos3);
        let mut input = Array::zeros((1, 3, 640, 640));
        for pixel in new_img.pixels() {
            let x = pixel.0 as _;
            let y = pixel.1 as _;
            let [r, g, b, _] = pixel.2 .0;
            input[[0, 0, y, x]] = (r as f32) / 255.;
            input[[0, 1, y, x]] = (g as f32) / 255.;
            input[[0, 2, y, x]] = (b as f32) / 255.;
        }
        input
    }

    fn process_output(&self, outputs: Tensor<f32>) -> Vec<Detection> {
        // 转置shape为[25200, 20]
        let output = outputs.view().t().slice(s![.., .., 0]).t().into_owned();
        let mut detections = Vec::new();
        for row in output.rows() {
            // 获取最大置信度类别的索引和置信度
            let mut class_id = 0;
            let mut max_prob = 0_f32;
            for idx in 5..20 {
                let prob = row[idx];
                if prob > max_prob {
                    // Subtract 5 to get the correct class index
                    class_id = idx - 5;
                    max_prob = prob;
                }
            }
            // 置信度 * 类别置信度
            let conf = row[4] * max_prob;
            if conf < CONFIDENCE_THRESHOLD {
                continue;
            }
            let label = LABELS[class_id];
            detections.push(Detection::new(row[0], row[1], row[2], row[3], label, conf));
        }
        detections
    }
}

#[cfg(test)]
mod tests {
    use std::path;

    use tracing::info;
    use xcap::image::GenericImage;

    use super::*;
    use crate::chess;
    use crate::common;
    use crate::engine;
    use crate::logger;

    #[tokio::test]
    async fn test_predict() {
        logger::init_tracer();
        let p = path::PathBuf::from("/Users/atopx/script/chessboard/libs/model.onnx");
        let model = Model::new(p).unwrap();
        let window = common::get_windows("JJ象棋").unwrap();
        let mut eng = engine::Engine::new("/Users/atopx/script/chessboard/libs/pikafish");
        let mut image = window.capture_image().unwrap();
        let mut detections = model.predict(image.clone()).unwrap();
        info!("{}", detections.len());
        let (x, y, w, h) =
            common::detections_bound(image.width(), image.height(), &detections).unwrap();
        image = image.sub_image(x, y, w, h).to_image();
        image.save("test.png").unwrap();
        detections = model.predict(image).unwrap();
        let (camp, mut board) = common::detections_to_board(detections).unwrap();
        let mut fen = chess::board_fen(board);
        fen.push(' ');
        fen.push(camp.to_char());
        info!("fen {:?}", fen);
        let result = eng.go(&fen, 25, 2500).await.unwrap();
        for pv in result.pvs {
            let notice = chess::board_move_chinese(board, &pv);
            board = chess::board_move(board, &pv);
            info!("{:?}", notice);
        }
    }
}
