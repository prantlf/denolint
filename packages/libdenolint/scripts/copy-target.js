import { copyFile } from 'fs/promises'
import { dirname, join } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const { argv, platform } = process
const [, , cross] = argv

let libName
switch (platform.charAt(0)) {
  case 'w':
    libName = 'libdenolint.dll'
    break
  case 'd':
    libName = 'liblibdenolint.dylib'
    break
  default:
    libName = 'liblibdenolint.so'
}

const outDir = cross ? `${cross}/release` : 'release'

const source = join(__dirname, `../../../target/${outDir}/${libName}`)
const target = join(__dirname, '../libdenolint.node')
await copyFile(source, target)
