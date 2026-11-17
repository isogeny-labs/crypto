#!/usr/bin/env bash
# Fixed format/integration DEVELOPMENT checks. No unknown-solving performance study.
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
study=$(cd "$here/../.." && pwd)
build=${1:-/tmp/norm-canonical-check}
mkdir -p "$build" "$here/outputs"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror "$here/canonicalize.cpp" -o "$build/canonicalize"
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic -Werror "$study/independent-check/check_export.cpp" -o "$build/check_export"
"$build/canonicalize" --selftest
for source in "$here"/inputs/*.ms; do
    name=$(basename "$source" .ms)
    "$build/canonicalize" "$source" "$here/outputs/$name.ms" --assignment "${source%.ms}.assignment.tsv" --products-output "$here/outputs/$name.products.ms"
    "$build/check_export" "$here/outputs/$name.products.ms" "${source%.ms}.assignment.tsv"
done
if [[ $# -lt 2 ]]; then
    echo 'PASS adapter checks; external backend not requested'
    exit 0
fi
msolve=$2
"$msolve" -h > "$build/msolve-help.txt" 2>&1
head -3 "$build/msolve-help.txt"
for input in "$here"/outputs/*.ms; do
    case "$input" in *.products.ms) continue;; esac
    name=$(basename "$input" .ms)
    # Bind all variables to the known fixture for a parser/integration check only.
    # The raw +0 constants are normalized again before invoking the CAS.
    awk 'FNR==NR{line[++n]=$0;next} FNR>1{c[++k]=$1"+"((17-$2)%17)} END{for(i=1;i<=n;i++)print line[i](i==n?",":"");for(i=1;i<=k;i++)print c[i](i<k?",":"")}' "$input" "$here/inputs/$name.assignment.tsv" > "$build/$name.bound.ast"
    "$build/canonicalize" "$build/$name.bound.ast" "$build/$name.bound.ms" > "$build/$name.bound.json"
    timeout 20 "$msolve" -t 1 -g 2 -f "$build/$name.bound.ms" -o "$build/$name.gb"
    awk 'FNR==NR{if(FNR>1){want[$1]=$2;nv++}next} /^#/{next} {gsub(/\[|\]|:|,/, "");gsub(/[[:space:]]/, "");if($0=="")next;a=$0;if(a!~/^1\*[a-zA-Z_][a-zA-Z_0-9]*\^1(\+[0-9]+)?$/)exit 2;sub(/^1\*/,"",a);split(a,b,"\\^1");v=b[1];c=b[2];sub(/^\+/,"",c);if(!(v in want)||seen[v]++||((17-(c+0))%17)!=want[v])exit 3;n++} END{if(n!=nv)exit 4}' "$here/inputs/$name.assignment.tsv" "$build/$name.gb"
    echo "PASS msolve bound-witness import $name"
done
echo 'PASS six canonical imports; known assignments only; no timing claim'
