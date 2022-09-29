const { mkdirSync, readFileSync, symlinkSync } = require('fs')
const { join } = require('path')

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
  const exeSuffix = platform === 'win32' ? '.exe' : ''
  const exeName = `denolint${exeSuffix}`
  const modDir = `${env.INIT_CWD}/node_modules`
  const binDir = join(modDir, '.bin')
  const link = join(binDir, exeName)
  mkdirSync(binDir, { recursive: true })
  const target = join(modDir, `@denolint/denolint-${platform}-${suffix()}/${exeName}`)
  symlinkSync(target, link, 'junction')
}
