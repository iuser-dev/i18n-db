//!/usr/bin/env coffee
var ROOT, i;

import lib from './lib.js';

import {
  writeFile
} from 'fs/promises';

import {
  join,
  dirname
} from 'path';

ROOT = dirname(decodeURI(new URL(import.meta.url).pathname));

await writeFile(join(ROOT, 'index.js'), `import lib from "./lib.js";
export const {${((function() {
  var results;
  results = [];
  for (i in lib) {
    results.push(i);
  }
  return results;
})()).join(',')}} = lib;`);
