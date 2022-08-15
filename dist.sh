#!/usr/bin/env bash

_DIR=$(cd "$(dirname "$0")"; pwd)

cd $_DIR

set -ex

bun run cep -- -c .

./build.sh

git add -A && git commit -m'~' || true

version=$(cat package.json|jq -r '.version')

git pull

npm set unsafe-perm true
npm version patch
git push
#npm publish --access=public
