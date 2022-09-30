/* eslint-disable no-unused-vars, prettier/prettier */

interface DenoLintOptions {
  scanDirs?: string[],
  ignorePatterns?: string[],
}

export function denolint(
  projectDir?: string, configPath?: string,
  options?: DenoLintOptions, signal?: AbortSignal
): Promise<boolean>

export function denolintSync(
  projectDir?: string, configPath?: string, options?: DenoLintOptions
): boolean

export function lint(
  fileName: string, sourceCode: string | Buffer,
  allRules?: boolean, excludeRules?: string[], includeRules?: string[],
  signal?: AbortSignal
): Promise<Array<string>>

export function lintSync(
  fileName: string, sourceCode: string | Buffer,
  allRules?: boolean, excludeRules?: string[], includeRules?: string[]
): Array<string>
