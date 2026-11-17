#!/usr/bin/env bash
set -euo pipefail
: "${MSOLVE_SOURCE:?unmodified pinned source tree}"
: "${DEPS:?extracted dependency prefix}"
: "${DIAGNOSTIC_BUILD:?new output directory}"
python3 - "$MSOLVE_SOURCE" <<'CHECK'
import hashlib,sys,tarfile
from pathlib import Path
p=Path(sys.argv[1]);a=p.parent/'msolve-0.6.5.tar.gz'
assert hashlib.sha256(a.read_bytes()).hexdigest()=='30e3712eab4077438e2b3228d5958adb7e8488e523244a02320f80bd8c412c89'
with tarfile.open(a) as t:
 for m in t.getmembers():
  rel=Path(m.name).parts[1:]
  if m.isfile() and rel and rel[0]=='src':
   assert t.extractfile(m).read()==p.joinpath(*rel).read_bytes(),m.name
CHECK
mkdir -p "$DIAGNOSTIC_BUILD"
printf '/* Scalar diagnostic configuration; optional SIMD disabled. */\n' > "$DIAGNOSTIC_BUILD/config.h"
cd "$MSOLVE_SOURCE"
gcc -O1 -g -fno-omit-frame-pointer -fsanitize=address,undefined -fno-sanitize-recover=all \
  -fopenmp -DHAVE_CONFIG_H=1 "-I$DIAGNOSTIC_BUILD" "-I$DEPS/usr/include" \
  "-L$DEPS/usr/lib/x86_64-linux-gnu" "-Wl,-rpath,$DEPS/usr/lib/x86_64-linux-gnu" \
  src/msolve/main.c src/neogb/gb.c src/fglm/fglm_core.c src/usolve/usolve.c \
  -o "$DIAGNOSTIC_BUILD/msolve" -lflint -lmpfr -lgmp -lm
