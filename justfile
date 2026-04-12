image := "ghcr.io/crust-gather/crust-gather:latest"

create-kind:
    kind create cluster

load-kind-image:
    docker build -t {{image}} .
    kind load docker-image {{image}}
