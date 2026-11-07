#!/bin/sh
# usage: build_wdsat.sh OUT MAXID   (pinned sources copied to src/wdsat_src; only __MAX_ID__ changed)
set -eu
HERE=$(cd "$(dirname "$0")" && pwd); OUT="$1"; ID="$2"
rm -rf "$OUT"; mkdir -p "$OUT"; cp -R "$HERE/wdsat_src" "$OUT/s"
python3 - "$OUT/s/config.h" "$ID" <<'PY'
import sys,re
p,i=sys.argv[1],sys.argv[2]
s=open(p).read()
s=re.sub(r'(#define __MAX_ID__ )767',r'\g<1>'+i,s,count=1)
s=re.sub(r'(#define __MAX_BUFFER_SIZE__ )60000',r'\g<1>200000',s,count=1)
s=re.sub(r'(#define __MAX_EQ__ )3000',r'\g<1>8000',s,count=1)
s=re.sub(r'(#define __MAX_ANF_ID__ )52',r'\g<1>72',s,count=1)
s=re.sub(r'(#define __MAX_XEQ__ )52',r'\g<1>120',s,count=1)
s=re.sub(r'(#define __MAX_XEQ_SIZE__ )800',r'\g<1>3000',s,count=1)
open(p,'w').write(s)
PY
(cd "$OUT/s" && cc -O3 -w *.c -lm -o ../wdsat_solver)
