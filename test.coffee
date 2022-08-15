#!/usr/bin/env coffee

> ./index.js > Db


db = new Db('/tmp/db')


db.transSet('en', '测试', 'test1')
console.log db.trans('en','测试')

###
console.log 1111

console.log db.fileIsChange(
  '/Users/z/iuser/i18n/i18n-db'
  'test.coffee'
  (txt)=>
    new Promise (resolve)=>
      console.log '>>> resolve'
      resolve()
      console.log txt
      return
)


console.log db

console.log 2222
###
