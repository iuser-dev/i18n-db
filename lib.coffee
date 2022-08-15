> fs > existsSync readFileSync
  path > join dirname
  module > createRequire
  ./wrap.js

{platform, arch}  = process

suffix = "#{platform}-#{arch}"

switch platform
  when 'android'
    if arch == 'arm'
      suffix += '-eab'
  when 'win32'
    suffix += '-msvc'
  when 'linux'
    if arch == 'arm'
      suffix += '-gnueabihf'
    else
      if process.report.getReport().header.glibcVersionRuntime
        suffix += '-gnu'
      else
        suffix += '-musl'

dot_node = "i18n-db.#{suffix}.node"

export default wrap createRequire(import.meta.url)(
  if existsSync(
    join(
      dirname(decodeURI new URL(import.meta.url).pathname)
      dot_node
    )
  ) then './'+dot_node else "@iuser/i18n-db-#{suffix}"
)

