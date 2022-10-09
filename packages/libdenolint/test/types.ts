import { denolint, denolintSync, lint, lintSync } from 'libdenolint'

declare type testCallback = () => void
// eslint-disable-next-line no-unused-vars
declare function test(label: string, callback: testCallback): void

test('Type declarations for TypeScript', () => {
  const aborter = new AbortController()

  denolint()
  denolint('')
  denolint('', '')
  denolint('', '', {})
  denolint('', '', {})
  denolint('', '', { scanDirs: [] })
  denolint('', '', { scanDirs: [''] })
  denolint('', '', { ignorePatterns: [] })
  denolint('', '', { ignorePatterns: [''] })
  denolint('', '', { format: 'compact' })
  denolint('', '', { format: 'pretty' })
  denolint('', '', { dryRun: true })
  denolint('', '', {}, aborter.signal)

  denolintSync()
  denolintSync('')
  denolintSync('', '')
  denolintSync('', '', {})
  denolintSync('', '', {})
  denolintSync('', '', { scanDirs: [] })
  denolintSync('', '', { scanDirs: [''] })
  denolintSync('', '', { ignorePatterns: [] })
  denolintSync('', '', { ignorePatterns: [''] })
  denolintSync('', '', { format: 'compact' })
  denolintSync('', '', { format: 'pretty' })
  denolintSync('', '', { dryRun: true })

  lint('', '')
  lint('', Buffer.from(''))
  lint('', '', {})
  lint('', '', { allRules: true })
  lint('', '', { excludeRules: [] })
  lint('', '', { excludeRules: [''] })
  lint('', '', { includeRules: [] })
  lint('', '', { includeRules: [''] })
  lint('', '', { format: 'compact' })
  lint('', '', { format: 'pretty' })
  lint('', '', {}, aborter.signal)

  lintSync('', '')
  lintSync('', Buffer.from(''))
  lintSync('', '', {})
  lintSync('', '', { allRules: true })
  lintSync('', '', { excludeRules: [] })
  lintSync('', '', { excludeRules: [''] })
  lintSync('', '', { includeRules: [] })
  lintSync('', '', { includeRules: [''] })
  lintSync('', '', { format: 'compact' })
  lintSync('', '', { format: 'pretty' })
})
