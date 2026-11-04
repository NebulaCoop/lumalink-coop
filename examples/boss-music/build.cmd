@echo off
rem Builds plugins\boss_music.dll with clang, from https://llvm.org. It needs nothing else.
if not exist plugins mkdir plugins
clang --target=x86_64-pc-windows-msvc -O2 -shared -fuse-ld=lld -nostdlib -Wl,/NOENTRY -Isource -o plugins\boss_music.dll source\plugin.c
