#!/bin/sh

set -e
set -x

pnpm -r publish --no-git-checks

for dir in packages/denolint/npm/*; do
  cd packages/denolint/npm/win32-x64-msvc
  pnpm publish --no-git-checks
  cd ../../../..
done

for dir in packages/libdenolint/npm/*; do
  cd packages/libdenolint/npm/win32-x64-msvc
  pnpm publish --no-git-checks
  cd ../../../..
done
