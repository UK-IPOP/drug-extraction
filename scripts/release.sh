#!/usr/bin/env bash

echo "Initializing new release..."

# export .env file

if [ ! -f .env ]
then
    export $(cat ../.env | xargs)
fi


git stash 

read -p "Tag: " tag
read -p "Message: " msg

echo "Releasing $tag"

git tag -a "$tag" -m "$msg"
git push origin "$tag"

goreleaser check
goreleaser release --rm-dist

git stash pop