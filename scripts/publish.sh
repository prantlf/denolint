#!/bin/sh

set +e
set -x

pnpm -r publish --no-git-checks

for dir in packages/denolint/npm/*; do
  cd $dir
  pnpm publish --no-git-checks
  cd ../../../..
done

for dir in packages/libdenolint/npm/*; do
  cd $dir
  pnpm publish --no-git-checks
  cd ../../../..
done
