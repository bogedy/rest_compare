from fastapi import FastAPI
from ml import onnx_model

model = onnx_model("../tinybert-imdb", "model.onnx")
app = FastAPI()

@app.get("/")
async def read_root():
    return {"Hello": "World", "backend": "FastAPI"}

# Define a new path operation that includes a path parameter
@app.get("/{query_text}")
async def process_text(query_text: str):
    # Placeholder for processing the query_text
    output = model(query_text)[0][0].tolist()
    return {"model_out": output}

# To run this FastAPI app, use the following command in the terminal:
# uvicorn filename:app --reload
# Replace 'filename' with the name of your Python script without the '.py' extension.