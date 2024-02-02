#!/bin/bash

set -e

# Ref: https://stackoverflow.com/questions/3404936/show-which-git-tag-you-are-on
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

source "$SCRIPT_DIR"/../.env
docker_image_reponame="${COMPOSE_PROJECT_NAME}/${PROJECT_DIRNAME}"
echo "docker_image_reponame --> '$docker_image_reponame'"

## docker image version
git_version="$(git describe --tags --exact-match --match "v*.*.*" ||
    git describe --match "v*.*.*" --tags || git describe --tags || git rev-parse HEAD)"
current_tagname="$docker_image_reponame":"$git_version"
latest_tagname="$docker_image_reponame":latest
echo "Building $current_tagname"

## Before process
# Update the submodule for later use
# git submodule update --init --recursive

## Process
DOCKER_BUILDKIT=1 docker build -t "$current_tagname" "$SCRIPT_DIR"/../
docker tag "$current_tagname" "$latest_tagname"

echo
echo "Tagging $current_tagname --> $latest_tagname"
echo "Job done!"
