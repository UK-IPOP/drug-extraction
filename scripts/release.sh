#!/usr/bin/env bash

echo "Initializing new release..."

# export .env file

export $(cat .env | xargs)
export GIN_MODE=release

git stash 

read -p "Tag: " tag
read -p "Message: " msg

echo "Releasing $tag"

git tag -a "$tag" -m "$msg"
git push origin "$tag"

goreleaser check
goreleaser release --rm-dist

git stash pop
export GIN_MODE=debug