# Define default make action
.PHONY: all
all: buildrust buildpython

# Build Docker image for implementation1
.PHONY: buildrust
buildrust:
	docker build -t rust_actix-image -f rust_actix/Dockerfile .

# Run Docker container for implementation1
.PHONY: runrust
runrust:
	docker run -d -p 8082:8082 --name rust_actix-container rust_actix-image

# Build Docker image for implementation2
.PHONY: buildpython
buildpython:
	docker build -t python-image -f python/Dockerfile .

# Run Docker container for implementation2
.PHONY: runpython
runpython:
	docker run -d -p 8000:8000 --name python-container python-image

# Clean up Docker containers and images
.PHONY: clean
clean:
	docker rm -f rust_actix-container python-container
	docker rmi rust_actix-image python-image

# push docker image to gcr
.PHONY: push
push:
	# Tag the Rust Actix image for GCP Artifact Registry
	docker tag rust_actix-image gcr.io/network-optimization-428815/rust_actix-image

	# Push the Rust Actix image to GCP Artifact Registry
	docker push gcr.io/network-optimization-428815/rust_actix-image

	# Tag the Python image for GCP Artifact Registry
	docker tag python-image gcr.io/network-optimization-428815/python-image

	# Push the Python image to GCP Artifact Registry
	docker push gcr.io/network-optimization-428815/python-image