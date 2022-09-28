import { createRequire } from 'module'

const require = createRequire(import.meta.url)
const { denolint, denolintSync, lint, lintSync } = require('../libdenolint.node')

export { denolint, denolintSync, lint, lintSync }
