#!/bin/bash
# mksrc.sh SHA OUT.tar.gz : minimal buildable crypto source tree at SHA:
# Cargo.toml + every path it references + every file named by include_str!/include_bytes!
# (resolved relative to the including file) in the archived trees.
set -euo pipefail
sha=$1; out=$2; cd /home/user/crypto
list=(Cargo.toml Cargo.lock src crates benches examples isogeny_algos)
for p in $(git show $sha:Cargo.toml | grep -oE 'path *= *"[^"]+"' | sed -E 's/.*"([^"]+)"/\1/'); do
  case $p in src/*|crates/*|benches/*|examples/*|isogeny_algos*) ;;
    *.rs) list+=("$(dirname "$p")");;
    *) list+=("$p");;
  esac
done
list+=($(git ls-tree --name-only $sha | grep -E '^(build\.rs|\.cargo|rust-toolchain.*)$' || true))
dirs=$(printf '%s\n' "${list[@]}" | sort -u)
inc=$(git grep -oE 'include_(str|bytes)!\(\s*"[^"]+"' $sha -- $dirs 2>/dev/null | python3 -c '
import sys,os,re,subprocess
sha=sys.argv[1]; out=set()
for l in sys.stdin:
    parts=l.rstrip("\n").split(":",2)
    if len(parts)<3: continue
    _,path,rest=parts
    rel=re.search(r"\"([^\"]+)\"",rest).group(1)
    out.add(os.path.normpath(os.path.join(os.path.dirname(path),rel)))
tree=set(subprocess.run(["git","ls-tree","-r","--name-only",sha],capture_output=True,text=True).stdout.split("\n"))
print("\n".join(sorted(p for p in out if p in tree)))' $sha)
git archive --format=tar.gz -o "$out" $sha -- $dirs $inc
