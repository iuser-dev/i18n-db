#!/usr/bin/env coffee

> ./index.js > Db


db = new Db('/tmp/db')


db.transSet('en', '测试', 'test1')
console.log db.trans('en','测试')


key = '''2022年4月，毫末智行宣布获得数亿元的A+轮融资，同时也将同步开启B轮融资计划，为第一轮融资划上圆满句号，也为企业发展史画上了浓烈重彩的一笔。同时，借助美团、高瓴、首程等知名投资方的持续看好，也巩固了龙头地位，成就其行业独角兽的荣誉。

值得注意的是，在这笔融资之前，2021年毫末智行一共进行了三轮融资，全年融资金额已然超过了13亿元。而且仅在2月份前后相隔10天左右，就完成了来自老东家长城汽车的第一轮融资，以及同样由美团、高瓴等领投的第二轮融
资。

随着自动驾驶技术经由出行平台在全国各地开启商业化落地测试，越来越多的资本机构开始对这一新兴行业寄予厚望。'''
val =   'In April 2022, Momo Zhixing announced that it had obtained hundreds of millions of yuan in A + round of financing, and at the same time, it will also start the B round of financing plan at the same time, marking a successful conclusion to the first round of financing and drawing a strong color for the history of corporate development. At the same time, with the continued optimism of well-known investors such as Meituan, Hillhouse, and Shoucheng, it has also consolidated its leading position and achieved the honor of a unicorn in its industry. \n' +
    '  \n' +
    ' It is worth noting that, prior to this financing, a total of three rounds of financing were carried out in 2021, and the annual financing amount has exceeded 1.3 billion yuan. And only about 10 days before and after February, the first round of financing from the old owner, Great Wall Motors, and the second round of financing also led by Meituan and Hillhouse were completed. \n' +
    '  \n' +
    ' As autonomous driving technology has been commercialized and tested across the country through travel platforms, more and more capital institutions have begun to place high hopes on this emerging industry.\n' +
    '\n' +
    '\n'
db.transSet('en', key, val)
console.log db.trans('en',key)

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
