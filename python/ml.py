import onnxruntime as ort
from tokenizers import Tokenizer
import numpy as np

class onnx_model:
    def __init__(self, model_dir, model_filename):
        self.model_path = f"{model_dir}/{model_filename}" 
        self.session = ort.InferenceSession(self.model_path)
        self.tokenizer = Tokenizer.from_file(model_dir+"/tokenizer.json")

    def tokenize(self, text):
        return self.tokenizer.encode(text)

    def predict(self, inputs):
        ort_inputs = {
            'input_ids': [inputs.ids],
            'attention_mask': [inputs.attention_mask],
            'token_type_ids': [inputs.type_ids]
        }
        outputs = self.session.run(None, ort_inputs)
        return outputs
    
    def __call__(self, text):
        inputs = self.tokenize(text)
        return self.predict(inputs)


if __name__ == "__main__":
    model = onnx_model("../tinybert-imdb", "model.onnx")
    print("tokenize:", (model.tokenize("Your example text here")))
    print("===============")
    print("predict:", model.predict(model.tokenize("Your example text here")))
    print(model("Your example text here")[0][0].tolist())