//!/usr/bin/env coffee
var utf8decoder;

import lang_map from './lang_map.js';

utf8decoder = new TextDecoder();

export default (mod) => {
  var Db, trans, transSet;
  ({Db} = mod);
  ({trans, transSet} = Db.prototype);
  Db.prototype.trans = function(...args) {
    var r;
    args[0] = lang_map.get(args[0]);
    r = trans.apply(this, args);
    if (r) {
      r = utf8decoder.decode(r);
    }
    return r;
  };
  Db.prototype.transSet = function(...args) {
    args[0] = lang_map.get(args[0]);
    return transSet.apply(this, args);
  };
  return mod;
};
