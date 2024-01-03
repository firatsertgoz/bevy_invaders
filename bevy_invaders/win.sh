#!/bin/sh
CARGO_FEATURE_PURE=1  cargo build --target x86_64-pc-windows-msvc &&
cp target/x86_64-pc-windows-msvc/debug/mygame.exe . &&
exec ./mygame.exe "$@"s