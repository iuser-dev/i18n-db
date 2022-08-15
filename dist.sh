#!/usr/bin/env bash

_DIR=$(cd "$(dirname "$0")"; pwd)

cd $_DIR

set -ex

bun run cep -- -c .

./build.sh

version=$(cat package.json|jq -r '.version')

git pull

npm set unsafe-perm true
npm version patch
add
git push
#npm publish --access=public
