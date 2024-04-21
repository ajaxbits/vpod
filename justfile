set positional-arguments := true

@deploy version:
    nom build .#oci-image
    docker load < result
    docker image tag vpod:$1 registry.fly.io/vpod:$1
    docker push registry.fly.io/vpod:$1
    sleep 5
    fly deploy --image registry.fly.io/vpod:$1
