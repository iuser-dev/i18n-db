//!/usr/bin/env coffee
var utf8decoder;

utf8decoder = new TextDecoder();

export default (mod) => {
  var Db, trans;
  ({Db} = mod);
  ({trans} = Db.prototype);
  Db.prototype.trans = function() {
    var r;
    r = trans.apply(this, arguments);
    if (r) {
      r = utf8decoder.decode(r);
    }
    return r;
  };
  return mod;
};
