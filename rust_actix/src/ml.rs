use ort::{GraphOptimizationLevel, OpenVINOExecutionProvider, Session, Result, Error};
use ort;
use tokenizers::tokenizer::Tokenizer;

pub struct OnnxModel {
    session: Session,
    tokenizer: Tokenizer,
}

impl OnnxModel {
    pub fn new(model_dir: &str, model_filename: &str) -> Result<OnnxModel> {
        let model_path = format!("{}/{}", model_dir, model_filename);
        let session = Session::builder()?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_execution_providers([OpenVINOExecutionProvider::default().build()])?
        .commit_from_file(model_path)?;

        let tokenizer = Tokenizer::from_file(format!("{}/{}", model_dir, "tokenizer.json"))?;
        
        Ok(OnnxModel { session, tokenizer })
    }

    pub fn tokenize(&self, text: &str) -> Result<tokenizers::Encoding> {
        Ok(self.tokenizer.encode(text, true)?)
    }


    fn reformat_arr_slice(&self, arr: &[u32]) -> Vec<i64> {
        arr.to_vec().iter().map(|&x| x as i64).collect()
    }

    pub fn predict_from_encoding(&self, encoding: tokenizers::Encoding) -> std::result::Result<Vec<f32>, Error> {
        let session_inputs = ort::inputs! {
            "input_ids" => ([1,512], self.reformat_arr_slice(encoding.get_ids())),
            "attention_mask" => ([1,512], self.reformat_arr_slice(encoding.get_attention_mask())),
            "token_type_ids" => ([1,512], self.reformat_arr_slice(encoding.get_type_ids())),
        }?;

        let session_out = self.session.run(session_inputs)?;
        let logits = session_out.get("logits").ok_or_else(|| Error::GetMapValueType(ort::ErrorInternal::Msg(String::from("logits key not found"))))?;
        let (_shape, data) = logits.try_extract_raw_tensor()?;

        Ok(data.to_vec())
    }

    pub fn predict_from_string(&self, text: &str) -> Result<Vec<f32>> {
        let encoding = self.tokenize(text)?;
        Ok(self.predict_from_encoding(encoding)?)
    }
}