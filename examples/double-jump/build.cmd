@echo off
rem Builds the plugin for every device the game runs on: Windows, Linux, Android and macOS, x64 and arm64 each.
rem It needs clang and lld, from https://llvm.org; nothing else.
lumalink build "%~dp0."
