#!/bin/sh

set +e
set -x

pkg="$1"

if [ "$pkg" == "both" ] || [ "$pkg" == "denolint" ] ; then
  for dir in packages/denolint/npm/*; do
    cd $dir
    pnpm publish --no-git-checks
    cd ../../../..
  done
fi

if [ "$pkg" == "both" ] || [ "$pkg" == "libdenolint" ] ; then
  for dir in packages/libdenolint/npm/*; do
    cd $dir
    pnpm publish --no-git-checks
    cd ../../../..
  done
fi
