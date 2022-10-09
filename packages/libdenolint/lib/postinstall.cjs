const { existsSync, readFileSync, renameSync } = require('fs')
const { join } = require('path')

const { platform, arch, env } = process
const log = !!env.LIBDENOLINT_DEBUG

const isMusl = () => {
  const { glibcVersionRuntime } = process.report.getReport().header
  return !glibcVersionRuntime
}

const platforms = {
  android: {
    arm64: () => 'arm64',
    arm: () => 'arm-eabi',
  },
  win32: {
    x64: () => 'x64-msvc',
    ia32: () => 'ia32-msvc',
    arm64: () => 'arm64-msvc',
  },
  darwin: {
    x64: () => 'x64',
    arm64: () => 'arm64',
  },
  freebsd: {
    x64: () => 'x64',
  },
  linux: {
    x64: () => (isMusl() ? 'x64-musl' : 'x64-gnu'),
    arm64: () => (isMusl() ? 'arm64-musl' : 'arm64-gnu'),
    arm: () => 'arm-gnueabihf',
  },
}

const pkgFile = join(__dirname, '../package.json')
const { optionalDependencies } = JSON.parse(readFileSync(pkgFile, 'utf8'))
if (optionalDependencies) {
  log && console.log('[libdenolint] Release installation detected.')
  const archs = platforms[platform]
  if (!archs) throw new Error(`Unsupported platform: ${platform}`)
  const suffix = archs[arch]
  if (!suffix) throw new Error(`Unsupported architecture: ${arch}`)
  log && console.log(`[libdenolint] Platform ${platform}, architecture ${arch}, suffix ${suffix()}.`)
  const target = join(__dirname, '../libdenolint.node')
  if (!existsSync(target)) {
    const source = require.resolve(`@denolint/libdenolint-${platform}-${suffix()}`)
    log && console.log(`[libdenolint] Moving "${source}" to "${target}".`)
    renameSync(source, target)
  } else {
    log && console.log(`[libdenolint] "${target}" already exists.`)
  }
} else {
  log && console.log('[libdenolint] Development installation detected.')
}
