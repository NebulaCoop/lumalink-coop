@echo off
rem Builds plugins\double_jump.dll with clang, from https://llvm.org. It needs nothing else.
clang --target=x86_64-pc-windows-msvc -O2 -shared -fuse-ld=lld -nostdlib -Wl,/NOENTRY -Isource -o plugins\double_jump.dll source\plugin.c
