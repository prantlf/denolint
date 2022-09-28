import { ok, rejects, strictEqual, throws } from 'assert'
import { dirname, join } from 'path'
import { fileURLToPath } from 'url'
import { readFile } from 'fs/promises'
import tehanu from 'tehanu'
import { denolint, denolintSync, lint, lintSync } from '../lib/index.mjs'

const test = tehanu(import.meta.url)

let __dirname, files, pass, warn, fail

test.before(async () => {
  __dirname = dirname(fileURLToPath(import.meta.url))
  files = join(__dirname, '../../../crates/shared/test')
  pass = await readFile(join(files, 'pass/ultimate.js'), 'utf8')
  warn = await readFile(join(files, 'warn/ultimate.js'), 'utf8')
  fail = await readFile(join(files, 'fail/ultimate.js'), 'utf8')
})

test('exports all', () => {
  strictEqual(typeof lintSync, 'function')
  strictEqual(typeof denolintSync, 'function')
})

test('async lint passes', async () => {
  strictEqual((await lint('pass.js', pass)).length, 0)
})

test('async lint warns', async () => {
  strictEqual((await lint('warn.js', warn)).length, 2)
})

test('async lint fails', async () => {
  rejects(() => lint('fail.js', fail))
})

test('sync lint passes', () => {
  strictEqual(lintSync('pass.js', pass).length, 0)
})

test('sync lint warns', () => {
  strictEqual(lintSync('warn.js', warn).length, 2)
})

test('sync lint fails', () => {
  throws(() => lintSync('fail.js', fail))
})

test('async denolint passes', async () => {
  ok(await denolint(join(files, 'pass')))
})

test('async denolint warns', async () => {
  ok(!await denolint(join(files, 'warn')))
})

test('async denolint fails', async () => {
  ok(!await denolint(join(files, 'fail')))
})

test('sync denolint passes', async () => {
  ok(denolintSync(join(files, 'pass')))
})

test('sync denolint warns', async () => {
  ok(!denolintSync(join(files, 'warn')))
})

test('sync denolint fails', async () => {
  ok(!denolintSync(join(files, 'fail')))
})
