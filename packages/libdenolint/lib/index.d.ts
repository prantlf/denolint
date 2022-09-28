/* eslint-disable no-unused-vars, prettier/prettier */

export function denolint(
  projDir?: string, configPath?: string, scanDirs?: string[],
  signal?: AbortSignal
): Promise<boolean>

export function denolintSync(
  projDir?: string, configPath?: string, scanDirs?: string[]
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
