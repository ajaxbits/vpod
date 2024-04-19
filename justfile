set positional-arguments

@deploy version:
    nix build .#oci-image
    docker load < result
    docker image tag vpod:$1 registry.fly.io/vpod:$1
    docker push registry.fly.io/vpod:$1
    fly deploy --image registry.fly.io/vpod:$1
