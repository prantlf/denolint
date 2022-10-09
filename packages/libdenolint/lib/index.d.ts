/* eslint-disable no-unused-vars, prettier/prettier */

interface DenoLintOptions {
  scanDirs?: string[],
  ignorePatterns?: string[],
  format?: 'compact' | 'pretty',
  dryRun?: boolean
}

export function denolint(
  projectDir?: string, configPath?: string,
  options?: DenoLintOptions, signal?: AbortSignal
): Promise<boolean>

export function denolintSync(
  projectDir?: string, configPath?: string, options?: DenoLintOptions
): boolean

interface LintOptions {
  allRules?: boolean,
  excludeRules?: string[],
  includeRules?: string[],
  format?: 'compact' | 'pretty'
}

export function lint(
  fileName: string, sourceCode: string | Buffer, options?: LintOptions,
  signal?: AbortSignal
): Promise<Array<string>>

export function lintSync(
  fileName: string, sourceCode: string | Buffer, options?: LintOptions
): Array<string>
