const { chmodSync, lstatSync, mkdirSync, readFileSync, symlinkSync, unlinkSync, writeFileSync } = require('fs')
const { basename, join } = require('path')

const { platform, arch, env } = process
const log = !!env.DENOLINT_DEBUG

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
  log && console.log('[denolint] Release installation detected.')
  const archs = platforms[platform]
  if (!archs) throw new Error(`Unsupported platform: ${platform}`)
  const suffix = archs[arch]
  if (!suffix) throw new Error(`Unsupported architecture: ${arch}`)
  log && console.log(`[denolint] Platform ${platform}, architecture ${arch}, suffix ${suffix()}.`)
  const platDir = `denolint-${platform}-${suffix()}`
  const binDir = join(`${env.INIT_CWD}/node_modules`, '.bin')
  log && console.log(`[denolint] Creating "${binDir}".`)
  mkdirSync(binDir, { recursive: true })
  if (platform !== 'win32') installBin(platDir, binDir)
  else installExe(platDir, binDir)
} else {
  log && console.log('[denolint] Development installation detected.')
}

function installBin(platDir, binDir) {
  const target = require.resolve(`@denolint/${platDir}`)
  log && console.log(`[denolint] Making "${target}" executable.`)
  chmodSync(target, 0o755)
  const link = join(binDir, basename(target))
  const stat = lstatSync(link, { throwIfNoEntry: false })
  if (stat) {
    log && console.log(`[denolint] Removing previous "${link}".`)
    unlinkSync(link)
  }
  log && console.log(`[denolint] Creating "${link}".`)
  symlinkSync(target, link)
}

function installExe(platDir, binDir) {
  const sh = `#!/bin/sh
basedir=$(dirname "$(echo "$0" | sed -e 's,\\\\,/,g')")
exec "$basedir/../@denolint/${platDir}/denolint.exe" "$@"
`
  const cmd = `@ECHO off
GOTO start
:find_dp0
SET dp0=%~dp0
EXIT /b
:start
SETLOCAL
CALL :find_dp0
ENDLOCAL & "%dp0%\\..\\@denolint\\${platDir}\\denolint.exe" %*
`
  const ps1 = `#!/usr/bin/env pwsh
$basedir=Split-Path $MyInvocation.MyCommand.Definition -Parent
if ($MyInvocation.ExpectingInput) {
  $input | & "$basedir/../@denolint/${platDir}/denolint.exe" $args
} else {
  & "$basedir/../@denolint/${platDir}/denolint.exe" $args
}
exit $LASTEXITCODE
`
  const base = join(binDir, 'denolint')
  writeScript('', sh, { mode: 0o755 })
  writeScript('.cmd', cmd)
  writeScript('.ps1', ps1)

  function writeScript(ext, text, options) {
    const script = `${base}${ext}`
    log && console.log(`[denolint] Creating "${script}".`)
    writeFileSync(script, text, options)
  }
}
