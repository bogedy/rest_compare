# TinyBERT-IMDb Endpoint

Rust and Python endpoints for a `tinybert-imdb` model, containerized and deployed on GCP. Benchmarked with k6 and monitored via Grafana. There are scripts for pulling the model and running the benchmarks and a makefile for the docker images. If you want to run this yourself remember to chagne the endpoint URLs in the k6 script and docker repo in the makefile. 

Thanks to [jkehres/docker-compose-influxdb-grafana](https://github.com/jkehres/docker-compose-influxdb-grafana) for the quick and easy Grafana config with Docker Compose! But you need to change InfluxDB to version 1.x, >2.0 doesn't work with k6 out of the box.

![Performance Screenshot](/assets/Screenshot.png)