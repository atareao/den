user    := "atareao"
name    := `basename ${PWD}`
version := `git tag -l  | tail -n1`

build:
    echo {{version}}
    echo {{name}}
    docker build -t {{user}}/{{name}}:{{version}} \
                 -t {{user}}/{{name}}:latest \
                 .

tag:
    docker tag {{user}}/{{name}}:{{version}} {{user}}/{{name}}:latest

push:
    docker push --all-tags {{user}}/{{name}}

upgrade:
    #!/bin/fish
    vampus upgrade --patch
    set VERSION $(vampus show)
    cargo update
    git commit -am "Upgrade to version $VERSION"
    git tag -a "$VERSION" -m "Version $VERSION"
    # clean old docker images
    docker image list  | grep {{name}} | sort -r | tail -n +5 | awk '{print $3}' | while read id; echo $id; docker rmi $id; end
    just build push

buildx:
    #!/usr/bin/env bash
    #--platform linux/arm/v7,linux/arm64/v8,linux/amd64 \
    docker buildx build \
           --push \
           --platform linux/arm/v7,linux/arm64/v8,linux/amd64 \
           --tag {{user}}/{{name}}:{{version}} .

run:
    docker run --rm \
               --init \
               --name croni \
               --init \
               --env_file croni.env \
               -v ${PWD}/crontab:/crontab \
               {{user}}/{{name}}:{{version}}

sh:
    docker run --rm \
               -it \
               --name croni \
               --init \
               --env-file croni.env \
               -v ${PWD}/crontab:/crontab \
               {{user}}/{{name}}:{{version}} \
               sh

