#!/usr/bin/env bash
set -e
DIR=$( dirname $(realpath "$0") )

cd $DIR

if [ ! -n "$1" ] ;then
exe=src/index.coffee
else
exe=${@:1}
fi

exec watchexec --shell=none \
  -w ./src \
  -w ./lib.coffee \
  -w ./test.coffee \
  -c \
  --exts coffee,rs \
  -r \
  -- ./run.sh
