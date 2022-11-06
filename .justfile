test:
    cargo test -- --nocapture
build:
    #!/usr/bin/env bash
    set -euxo pipefail
    name=$(grep -oP '^name\s*=\s*"\K([^"]*)' Cargo.toml)
    version=$(grep -oP '^version\s*=\s*"\K([^"]*)' Cargo.toml)
    docker build -t "atareao/${name}:v${version}" .

latest:
    #!/usr/bin/env bash
    set -euxo pipefail
    name=$(grep -oP '^name\s*=\s*"\K([^"]*)' Cargo.toml)
    version=$(grep -oP '^version\s*=\s*"\K([^"]*)' Cargo.toml)
    docker image tag "atareao/${name}:v${version}" "atareao/${name}":latest
    docker push "atareao/${name}:latest"

push:
    #!/usr/bin/env bash
    set -euxo pipefail
    name=$(grep -oP '^name\s*=\s*"\K([^"]*)' Cargo.toml)
    version=$(grep -oP '^version\s*=\s*"\K([^"]*)' Cargo.toml)
    docker push "atareao/$name:v${version}"

run:
    #!/usr/bin/env bash
    set -euxo pipefail
    name=$(grep -oP '^name\s*=\s*"\K([^"]*)' Cargo.toml)
    version=$(grep -oP '^version\s*=\s*"\K([^"]*)' Cargo.toml)
    docker run -it --rm --init --env-file .env --name "${name}" "atareao/${name}:v${version}"

buildx:
    #!/usr/bin/env bash
    #--platform linux/arm/v7,linux/arm64/v8,linux/amd64 \
    set -euxo pipefail
    name=$(grep -oP '^name\s*=\s*"\K([^"]*)' Cargo.toml)
    version=$(grep -oP '^version\s*=\s*"\K([^"]*)' Cargo.toml)
    docker buildx build \
           --push \
           --platform linux/arm/v7,linux/arm64/v8,linux/amd64 \
           --tag "atareao/${name}:v${version}" .
