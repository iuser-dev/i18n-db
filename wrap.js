//!/usr/bin/env coffee
var utf8decoder, wrap;

import lang_map from './lang_map.js';

utf8decoder = new TextDecoder();

wrap = (ob, key) => {
  var func;
  func = ob[key];
  ob[key] = function(...args) {
    args[0] = lang_map.get(args[0]);
    return func.apply(this, args);
  };
};

export default (mod) => {
  var Db, i, trans;
  ({Db} = mod);
  for (i in Db.prototype) {
    wrap(Db.prototype, i);
  }
  ({trans} = Db.prototype);
  Db.prototype.trans = function(...args) {
    var r;
    r = trans.apply(this, args);
    if (r) {
      r = utf8decoder.decode(r);
    }
    return r;
  };
  return mod;
};
