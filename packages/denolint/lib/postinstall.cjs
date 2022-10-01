const { chmodSync, mkdirSync, readFileSync, symlinkSync } = require('fs')
const { basename, join } = require('path')

const { platform, arch, env } = process

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
  const target = require.resolve(`@denolint/denolint-${platform}-${suffix()}`)
  if (platform !== 'win32') chmodSync(target, 0o755)
  const binDir = join(`${env.INIT_CWD}/node_modules`, '.bin')
  mkdirSync(binDir, { recursive: true })
  const link = join(binDir, basename(target))
  symlinkSync(target, link, 'junction')
}
