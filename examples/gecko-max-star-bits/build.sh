#!/bin/sh
# Builds the plugin for every device the game runs on: Windows, Linux, Android and macOS, x64 and arm64 each.
# It needs clang and lld, from https://llvm.org (a Mac: brew install llvm lld); nothing else.
set -e
lumalink build "$(dirname "$0")"
