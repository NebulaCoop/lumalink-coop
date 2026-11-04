@echo off
rem Builds plugins\little_boss.dll with clang, from https://llvm.org. It needs nothing else.
clang --target=x86_64-pc-windows-msvc -O2 -shared -fuse-ld=lld -nostdlib -Wl,/NOENTRY -Isource -o plugins\little_boss.dll source\plugin.c
