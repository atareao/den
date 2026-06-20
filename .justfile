user    := "atareao"
name    := `basename ${PWD}`
version := `vampus show`

build:
    @podman build \
        --tag={{user}}/{{name}}:{{version}} \
        --tag={{user}}/{{name}}:latest .

push:
    @podman image push --all-tags {{user}}/{{name}}

upgrade:
    #!/usr/bin/env bash
    set -euo pipefail
    vampus upgrade --patch
    VERSION=$(vampus show)
    cargo update
    git commit -am "Upgrade to version $VERSION"
    git tag -a "$VERSION" -m "Version $VERSION"
    podman image list | grep {{name}} | sort -r | tail -n +5 | awk '{print $3}' | while read -r id; do echo "$id"; podman rmi "$id"; done
    just build push

buildx:
    @podman build \
        --platform linux/arm/v7,linux/arm64/v8,linux/amd64 \
        --tag={{user}}/{{name}}:{{version}} \
        --tag={{user}}/{{name}}:latest .
    @podman image push --all-tags {{user}}/{{name}}

run:
    podman run --rm \
        --init \
        --name den \
        --env-file den.env \
        -v ${PWD}/crontab:/crontab \
        {{user}}/{{name}}:{{version}}

sh:
    podman run --rm \
        -it \
        --name den \
        --init \
        --env-file den.env \
        -v ${PWD}/crontab:/crontab \
        {{user}}/{{name}}:{{version}} \
        sh