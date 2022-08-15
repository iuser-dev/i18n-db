var arch, dot_node, platform, suffix;

import {
  existsSync,
  readFileSync
} from 'fs';

import {
  join,
  dirname
} from 'path';

import {
  createRequire
} from 'module';

import wrap from './wrap.js';

({platform, arch} = process);

suffix = `${platform}-${arch}`;

switch (platform) {
  case 'android':
    if (arch === 'arm') {
      suffix += '-eab';
    }
    break;
  case 'win32':
    suffix += '-msvc';
    break;
  case 'linux':
    if (arch === 'arm') {
      suffix += '-gnueabihf';
    } else {
      if (process.report.getReport().header.glibcVersionRuntime) {
        suffix += '-gnu';
      } else {
        suffix += '-musl';
      }
    }
}

dot_node = `i18n-db.${suffix}.node`;

export default wrap(createRequire(import.meta.url)(existsSync(join(dirname(decodeURI(new URL(import.meta.url).pathname)), dot_node)) ? './' + dot_node : `@iuser/i18n-db-${suffix}`));
