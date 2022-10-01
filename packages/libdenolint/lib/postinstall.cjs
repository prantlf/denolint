const { existsSync, readFileSync, renameSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

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
  const archs = platforms[platform]
  if (!archs) throw new Error(`Unsupported platform: ${platform}`)
  const suffix = archs[arch]
  if (!suffix) throw new Error(`Unsupported architecture: ${arch}`)
  const target = join(__dirname, '../libdenolint.node')
  if (!existsSync(target)) {
    const source = require.resolve(`@denolint/libdenolint-${platform}-${suffix()}`)
    renameSync(source, target)
  }
}
