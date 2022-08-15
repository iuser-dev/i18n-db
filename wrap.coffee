#!/usr/bin/env coffee

utf8decoder = new TextDecoder()

export default (mod)=>
  {Db} = mod
  {trans} = Db::
  Db::trans = ->
    r = trans.apply(@,arguments)
    if r
      r = utf8decoder.decode r
    r
  mod
