#!/bin/sh

set +e
set -x

pkg="$1"

publish() {
  cd $1
  pnpm publish --no-git-checks --access public
  cd ../../../..
}

if [ "$pkg" = "both" ] || [ "$pkg" = "denolint" ]; then
  for dir in packages/denolint/npm/*; do
    publish $dir
  done
fi

if [ "$pkg" = "both" ] || [ "$pkg" = "libdenolint" ]; then
  for dir in packages/libdenolint/npm/*; do
    publish $dir
  done
fi
