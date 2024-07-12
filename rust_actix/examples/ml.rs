use ort::{GraphOptimizationLevel, Session};
use ort;
use tokenizers::tokenizer::Tokenizer;

struct OnnxModel {
    session: Session,
    tokenizer: Tokenizer,
}

impl OnnxModel {
    fn new(model_dir: &str, model_filename: &str) -> Self {
        let model_path = format!("{}/{}", model_dir, model_filename);
        let session = Session::builder()
        .unwrap()
        .with_optimization_level(GraphOptimizationLevel::Level3)
        .unwrap()
        .with_intra_threads(4)
        .unwrap()
        .commit_from_file(model_path)
        .unwrap();

        let tokenizer = Tokenizer::from_file(format!("{}/{}", model_dir, "tokenizer.json")).unwrap();
        
        OnnxModel { session, tokenizer }
    }

    fn tokenize(&self, text: &str) -> tokenizers::Encoding {
        self.tokenizer.encode(text, true).unwrap()
    }


    fn _reformat_arr_slice(&self, arr: &[u32]) -> Vec<i64> {
        arr.to_vec().iter().map(|&x| x as i64).collect()
    }

    fn predict_from_encoding(&self, encoding: tokenizers::Encoding) -> Vec<f32> {
        let session_inputs = ort::inputs! {
            "input_ids" => ([1,512], self._reformat_arr_slice(encoding.get_ids())),
            "attention_mask" => ([1,512], self._reformat_arr_slice(encoding.get_attention_mask())),
            "token_type_ids" => ([1,512], self._reformat_arr_slice(encoding.get_type_ids())),
        }.unwrap();

        let session_out = self.session.run(session_inputs).unwrap();
        let logits = session_out.get("logits").unwrap();
        let (_shape, data) = logits.try_extract_raw_tensor().unwrap();

        data.to_vec()
    }

    fn predict_from_string(&self, text: &str) -> Vec<f32> {
        let encoding = self.tokenize(text);
        self.predict_from_encoding(encoding)
    }
}

fn main() {
    let model: OnnxModel = OnnxModel::new("../tinybert-imdb", "model.onnx");
    // print!("{:?}", model.tokenize("Your example text here").get_ids().len());
    let pred = model.predict_from_string("Your example text here");
    println!("{:?}", pred);

    
    println!("{:?}", model.predict_from_string("I loved this movie"));
    println!("{:?}", model.predict_from_string("this movie sucks"));
}