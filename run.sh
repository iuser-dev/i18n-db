#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR
set -ex

#nr build-debug
#bun run cep -c index.coffee
#./test.coffee

./build.sh :debug
./test.coffee
