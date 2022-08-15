#!/usr/bin/env coffee

> ./lib.js
  fs/promises > writeFile
  path > join dirname

ROOT = dirname(
  decodeURI new URL(
    import.meta.url
  ).pathname
)


await writeFile(
  join ROOT,'index.js'
  """import lib from "./lib.js";
export const {#{(i for i of lib).join(',')}} = lib;"""
)
