#!/usr/bin/env bash

echo "Initializing new release..."

read -p -r "Tag: " tag
read -p -r "Message: " msg

echo "Releasing $tag"

git tag -a "$tag" -m "$msg"
git push origin "$tag"

goreleaser check
goreleaser release

