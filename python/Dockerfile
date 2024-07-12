# Use Python 3.11 image from Debian bookworm slim
FROM python:3.11-slim-bookworm

# Set the working directory in the container
WORKDIR /project/app

# Copy the current directory contents into the container at /app
COPY ./python/ .

COPY ./tinybert-imdb/ /project/tinybert-imdb/

# Install the required packages
RUN pip install --no-cache-dir fastapi uvicorn tokenizers onnxruntime-openvino "numpy<2.0"

EXPOSE 8000
# Command to run the uvicorn server
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--reload"]