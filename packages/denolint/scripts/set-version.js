import { readFile, writeFile } from 'fs/promises'
import { dirname, join } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))

const pkgFile = join(__dirname, '../package.json')
const { version } = JSON.parse(await readFile(pkgFile, 'utf8'))
const mainFile = join(__dirname, '../src/main.rs')
let main = await readFile(mainFile, 'utf8')
main = main.replace(/println!\("[.0-9]+"\)/g, `println!("${version}")`)
await writeFile(mainFile, main)
