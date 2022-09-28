import { chmod, rename } from 'fs/promises'
import { dirname, join } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const { platform } = process

const exeSuffix = platform === 'win32' ? '.exe' : ''
const exeName = `denolint${exeSuffix}`
const source = join(__dirname, `../../../artifacts/denolint/${exeName}`)
const target = join(__dirname, `../${exeName}`)
await rename(source, target)

if (platform !== 'win32') {
  await chmod(target, 0o755)
}
