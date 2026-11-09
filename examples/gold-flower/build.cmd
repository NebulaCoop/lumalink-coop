@echo off
rem Builds the plugin for every device Nebula runs on, with clang from https://llvm.org. It needs nothing else: the plugin
rem calls only the game, never a system library.
set FLAGS=-O2 -shared -nostdlib -fuse-ld=lld -fvisibility=hidden -Isource
clang --target=x86_64-pc-windows-msvc %FLAGS% -Wl,/NOENTRY -Wl,/NOIMPLIB -o plugins\windows-x64\gold_flower.dll source\plugin.c
clang --target=aarch64-pc-windows-msvc %FLAGS% -Wl,/NOENTRY -Wl,/NOIMPLIB -o plugins\windows-arm64\gold_flower.dll source\plugin.c
clang --target=x86_64-linux-gnu %FLAGS% -fPIC -o plugins\linux-x64\libgold_flower.so source\plugin.c
clang --target=aarch64-linux-gnu %FLAGS% -fPIC -o plugins\linux-arm64\libgold_flower.so source\plugin.c
clang --target=aarch64-linux-android24 %FLAGS% -fPIC -o plugins\android-arm64\libgold_flower.so source\plugin.c
clang --target=x86_64-linux-android24 %FLAGS% -fPIC -o plugins\android-x64\libgold_flower.so source\plugin.c
