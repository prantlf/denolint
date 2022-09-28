import { access, copyFile, readFile, writeFile } from 'fs/promises'
import { dirname, join } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))

const platforms = {
  'win32-x64-msvc': 'x86_64-pc-windows-msvc',
  'darwin-x64': 'x86_64-apple-darwin',
  'linux-x64-gnu': 'x86_64-unknown-linux-gnu',
  'win32-ia32-msvc': 'i686-pc-windows-msvc',
  'linux-arm-gnueabihf': 'armv7-unknown-linux-gnueabihf',
  'linux-x64-musl': 'x86_64-unknown-linux-musl',
  'linux-arm64-gnu': 'aarch64-unknown-linux-gnu',
  'darwin-arm64': 'aarch64-apple-darwin',
  'android-arm64': 'aarch64-linux-android',
  'android-arm-eabi': 'armv7-linux-androideabi',
  'freebsd-x64': 'x86_64-unknown-freebsd',
  'linux-arm64-musl': 'aarch64-unknown-linux-musl',
  'win32-arm64-msvc': 'aarch64-pc-windows-msvc',
}

function getExeSuffix(dir) {
  return dir.startsWith('w') ? '.exe' : ''
}

async function addReadme(dir) {
  const readmeFile = join(__dirname, '../npm', dir, 'README.md')
  let readme = await readFile(readmeFile, 'utf8')
  readme = readme
    .replace(/# `([^`]+)`/g, '# $1')
    .replace(/\*\*([^*]+)\*\*/g, '`$1`')
    .replace(/for `[^`]+`\.?/g, 'for `denolint`.')
  await writeFile(readmeFile, readme)
}

async function addBin(dir) {
  const pkgFile = join(__dirname, '../npm', dir, 'package.json')
  const pkg = JSON.parse(await readFile(pkgFile, 'utf8'))
  delete pkg.main
  const exeName = `denolint${getExeSuffix(dir)}`
  pkg.files = [exeName]
  pkg.bin = { denolint: exeName }
  await writeFile(pkgFile, JSON.stringify(pkg, undefined, 2))
}

async function copyBin(dir) {
  const exeName = `denolint${getExeSuffix(dir)}`
  const osDir = platforms[dir]
  const source = join(__dirname, `../../../artifacts/bindings-${osDir}/${exeName}`)
  try {
    await access(source)
    const target = join(__dirname, `../npm/${dir}/${exeName}`)
    await copyFile(source, target)
  } catch {
    console.warn(`${osDir} not available`)
  }
}

const readme = Object.keys(platforms).map(addReadme)
const added = Object.keys(platforms).map(addBin)
const copied = Object.keys(platforms).map(copyBin)
await Promise.all([...readme, ...added, ...copied])
