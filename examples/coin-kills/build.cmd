@echo off
rem Builds plugins\coin_kills.dll with clang, from https://llvm.org. It needs nothing else.
if not exist plugins mkdir plugins
clang --target=x86_64-pc-windows-msvc -O2 -shared -fuse-ld=lld -nostdlib -Wl,/NOENTRY -Isource -o plugins\coin_kills.dll source\plugin.c
