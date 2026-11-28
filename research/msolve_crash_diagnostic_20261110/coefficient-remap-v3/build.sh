#!/usr/bin/env bash
set -euo pipefail
: "${VARIANT:?logged or corrected}"
: "${MSOLVE_SOURCE:?pinned tarball tree}"
: "${DEPS:?dependency prefix}"
: "${DIAGNOSTIC_SOURCE:?new copied source path}"
: "${DIAGNOSTIC_BUILD:?new build path}"
case "$VARIANT" in logged|corrected) ;; *) exit 2;; esac
study=$(cd "$(dirname "$0")" && pwd)
python3 - "$MSOLVE_SOURCE" <<'CHECK'
import hashlib,sys,tarfile
from pathlib import Path
p=Path(sys.argv[1]);a=p.parent/'msolve-0.6.5.tar.gz'
assert hashlib.sha256(a.read_bytes()).hexdigest()=='30e3712eab4077438e2b3228d5958adb7e8488e523244a02320f80bd8c412c89'
with tarfile.open(a) as t:
 for m in t.getmembers():
  rel=Path(m.name).parts[1:]
  if m.isfile() and rel and rel[0]=='src':assert t.extractfile(m).read()==p.joinpath(*rel).read_bytes(),m.name
CHECK
[ ! -e "$DIAGNOSTIC_SOURCE" ]
mkdir -p "$DIAGNOSTIC_SOURCE" "$DIAGNOSTIC_BUILD"
cp -a "$MSOLVE_SOURCE/src" "$DIAGNOSTIC_SOURCE/src"
cd "$DIAGNOSTIC_SOURCE"
patch --batch --fuzz=0 -p1 < "$study/$VARIANT.patch"
printf '/* Scalar diagnostic configuration; optional SIMD disabled. */\n' > "$DIAGNOSTIC_BUILD/config.h"
gcc -O1 -g -fno-omit-frame-pointer -fsanitize=address -fno-sanitize-recover=all \
  -fopenmp -DHAVE_CONFIG_H=1 "-I$DIAGNOSTIC_BUILD" "-I$DEPS/usr/include" \
  "-L$DEPS/usr/lib/x86_64-linux-gnu" "-Wl,-rpath,$DEPS/usr/lib/x86_64-linux-gnu" \
  src/msolve/main.c src/neogb/gb.c src/fglm/fglm_core.c src/usolve/usolve.c \
  -o "$DIAGNOSTIC_BUILD/msolve" -lflint -lmpfr -lgmp -lm
