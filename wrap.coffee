#!/usr/bin/env coffee

> ./lang_map.js

utf8decoder = new TextDecoder()

wrap = (ob, key)=>
  func = ob[key]
  ob[key] = (...args)->
    args[0] = lang_map.get(args[0])
    func.apply(@,args)
  return

export default (mod)=>
  {Db} = mod
  for i of Db::
    wrap Db::, i

  {trans} = Db::
  Db::trans = (...args)->
    r = trans.apply(@,args)
    if r
      r = utf8decoder.decode r
    r

  mod
