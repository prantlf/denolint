import { rename } from 'fs/promises'
import { dirname, join } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))

const source = join(__dirname, '../../../artifacts/libdenolint/libdenolint.node')
const target = join(__dirname, '../libdenolint.node')
await rename(source, target)
