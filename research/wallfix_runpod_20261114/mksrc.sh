#!/bin/bash
# mksrc.sh SHA OUT.tar.gz : minimal buildable crypto source tree at SHA (Cargo.toml + every path it references)
set -euo pipefail
sha=$1; out=$2; cd /home/user/crypto
list=(Cargo.toml Cargo.lock src crates benches examples isogeny_algos)
for p in $(git show $sha:Cargo.toml | grep -oE 'path *= *"[^"]+"' | sed -E 's/.*"([^"]+)"/\1/'); do
  case $p in src/*|crates/*|benches/*|examples/*|isogeny_algos*) ;;
    *.rs) list+=("$(dirname "$p")");;   # bin/example file: take its dir (sibling modules)
    *) list+=("$p");;                    # path dependency crate dir
  esac
done
list+=($(git ls-tree --name-only $sha | grep -E '^(build\.rs|\.cargo|rust-toolchain.*)$' || true))
git archive --format=tar.gz -o "$out" $sha -- $(printf '%s\n' "${list[@]}" | sort -u)
