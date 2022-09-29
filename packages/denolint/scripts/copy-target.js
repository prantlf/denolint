import { copyFile } from 'fs/promises'
import { dirname, join } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const { argv, platform } = process
const [, , cross] = argv

const exeSuffix = platform === 'win32' ? '.exe' : ''
const exeName = `denolint${exeSuffix}`

const outDir = cross ? `${cross}/release` : 'release'

const source = join(__dirname, `../../../target/${outDir}/${exeName}`)
const target = join(__dirname, `../${exeName}`)
await copyFile(source, target)
