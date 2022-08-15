#!/usr/bin/env coffee

> ./lang_map.js

utf8decoder = new TextDecoder()

export default (mod)=>
  {Db} = mod
  {trans,transSet} = Db::
  Db::trans = (...args)->
    args[0] = lang_map.get(args[0])
    r = trans.apply(@,args)
    if r
      r = utf8decoder.decode r
    r
  Db::transSet = (...args)->
    args[0] = lang_map.get(args[0])
    transSet.apply(@,args)
  mod
