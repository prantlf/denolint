import { readFile, writeFile } from 'fs/promises'
import { dirname, join } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))

const pkgFile = join(__dirname, '../package.json')
const { version } = JSON.parse(await readFile(pkgFile, 'utf8'))

async function fixVer(file, source, target) {
  const path = join(__dirname, `../${file}`)
  let text = await readFile(path, 'utf8')
  text = text.replace(source, target)
  await writeFile(path, text)
}

await fixVer('src/main.rs', /println!\("[.0-9]+"\)/g, `println!("${version}")`)
await fixVer('Cargo.toml', /\nversion = "[.0-9]+"/g, `\nversion = "${version}"`)
