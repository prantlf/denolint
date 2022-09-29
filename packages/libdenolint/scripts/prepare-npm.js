import { readFile, rename, writeFile } from 'fs/promises'
import { dirname, join } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))

const platforms = {
  'win32-x64-msvc': 'x86_64-pc-windows-msvc',
  'darwin-x64': 'x86_64-apple-darwin',
  'linux-x64-gnu': 'x86_64-unknown-linux-gnu',
  // 'win32-ia32-msvc': 'i686-pc-windows-msvc',
  // 'linux-arm-gnueabihf': 'armv7-unknown-linux-gnueabihf',
  // 'linux-x64-musl': 'x86_64-unknown-linux-musl',
  // 'linux-arm64-gnu': 'aarch64-unknown-linux-gnu',
  'darwin-arm64': 'aarch64-apple-darwin',
  // 'android-arm64': 'aarch64-linux-android',
  // 'android-arm-eabi': 'armv7-linux-androideabi',
  // 'freebsd-x64': 'x86_64-unknown-freebsd',
  // 'linux-arm64-musl': 'aarch64-unknown-linux-musl',
  'win32-arm64-msvc': 'aarch64-pc-windows-msvc',
}

async function fixReadme(dir) {
  const readmeFile = join(__dirname, '../npm', dir, 'README.md')
  let readme = await readFile(readmeFile, 'utf8')
  readme = readme
    .replace(/# `([^`]+)`/g, '# $1')
    .replace(/\*\*([^*]+)\*\*/g, '`$1`')
    .replace(/for `[^`]+`\.?/g, 'for `libdenolint`.')
  await writeFile(readmeFile, readme)
}

async function fixFiles(dir) {
  const pkgFile = join(__dirname, '../npm', dir, 'package.json')
  const pkg = JSON.parse(await readFile(pkgFile, 'utf8'))
  pkg.files = ['libdenolint.node']
  pkg.main = 'libdenolint.node'
  await writeFile(pkgFile, JSON.stringify(pkg, undefined, 2))
}

async function moveLib(dir) {
  const source = join(__dirname, `../../../artifacts/${platforms[dir]}/libdenolint/libdenolint.node`)
  const target = join(__dirname, `../npm/${dir}/libdenolint.node`)
  await rename(source, target)
}

async function fixDeps() {
  const pkgFile = join(__dirname, '../package.json')
  const pkg = JSON.parse(await readFile(pkgFile, 'utf8'))
  const { version } = pkg
  pkg.optionalDependencies = Object.keys(platforms).reduce((deps, dir) => {
    deps[`@denolint/libdenolint-${dir}`] = version
    return deps
  }, {})
  await writeFile(pkgFile, JSON.stringify(pkg, undefined, 2))
}

const readme = Object.keys(platforms).map(fixReadme)
const added = Object.keys(platforms).map(fixFiles)
const copied = Object.keys(platforms).map(moveLib)
await Promise.all([...readme, ...added, ...copied, fixDeps()])
