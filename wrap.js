//!/usr/bin/env coffee
var utf8decoder;

import lang_map from './lang_map.js';

utf8decoder = new TextDecoder();

export default (mod) => {
  var Db, trans, transSet;
  ({Db} = mod);
  ({trans, transSet} = Db.prototype);
  Db.prototype.trans = function() {
    var r, to;
    to = arguments[0];
    arguments[0] = lang_map.get(to);
    r = trans.apply(this, arguments);
    if (r) {
      r = utf8decoder.decode(r);
    }
    return r;
  };
  Db.prototype.transSet = function() {
    var to;
    to = arguments[0];
    arguments[0] = lang_map.get(to);
    console.log(arguments);
    return transSet.apply(this, arguments);
  };
  return mod;
};
