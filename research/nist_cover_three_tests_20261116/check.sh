#!/bin/sh
set -eu
cd "$(dirname "$0")"
out=${1:?Specify a NEW output directory; existing evidence is never overwritten}
mkdir "$out"
out=$(cd "$out" && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT HUP INT TERM
cxx=${CXX:-c++}
flags='-std=c++17 -O2 -Wall -Wextra -Werror -pedantic'
$cxx $flags module_audit.cpp -o "$tmp/module-audit"
$cxx $flags point_replay.cpp -lgmpxx -lgmp -o "$tmp/point-replay"
sed '/^int main(/,$d' point_replay.cpp > "$tmp/point_arithmetic.hpp"
$cxx $flags -I . -I "$tmp" cover_replay.cpp -lgmpxx -lgmp -o "$tmp/cover-replay"
sed '/^int main(/,$d' cover_replay.cpp > "$tmp/cover_arithmetic.hpp"
$cxx $flags -I . -I "$tmp" kernel_point_replay.cpp -lgmpxx -lgmp -o "$tmp/kernel-point-replay"
$cxx $flags issue_identity.cpp -lgmpxx -lgmp -lcrypto -o "$tmp/issue-identity"
$cxx $flags make_control.cpp -o "$tmp/make-control"
sha256sum *.cpp json.hpp inputs/*.json "$tmp/"*_arithmetic.hpp "$tmp/module-audit" "$tmp/point-replay" "$tmp/cover-replay" "$tmp/kernel-point-replay" "$tmp/issue-identity" > "$out/pre-call-manifest.sha256"
chmod 0555 "$tmp/module-audit" "$tmp/point-replay" "$tmp/cover-replay" "$tmp/kernel-point-replay" "$tmp/issue-identity" "$tmp/make-control"
"$tmp/module-audit" ../nist_cover_separator_20261115/data "$out/kernels.jsonl" > "$out/modules.jsonl" 2> "$out/module-summary.txt"
"$tmp/point-replay" inputs/p192-d7-prym.json inputs/p192-d7-pairings.json inputs/p192-elliptic.json > "$out/point-replay.txt" 2> "$out/point-replay.stderr"
"$tmp/cover-replay" inputs/p192-d7-prym.json > "$out/cover-replay.txt" 2> "$out/cover-replay.stderr"
"$tmp/kernel-point-replay" inputs/p192-d7-prym.json inputs/p192-d7-pairings.json inputs/p192-elliptic.json "$out/kernels.jsonl" > "$out/actual-cover-torsion.json" 2> "$out/actual-cover-torsion-summary.txt"
mkdir "$out/identities"
"$tmp/issue-identity" "$out/identities" > "$out/identity-ids.txt"
"$tmp/make-control" kernel "$out/kernels.jsonl" "$tmp/bad-kernel.jsonl"
if "$tmp/kernel-point-replay" inputs/p192-d7-prym.json inputs/p192-d7-pairings.json inputs/p192-elliptic.json "$tmp/bad-kernel.jsonl" > "$out/control-kernel.stdout" 2> "$out/control-kernel.stderr"; then echo 'ERROR accepted non-isotropic kernel' >&2; exit 1; fi
printf '%s\n' 'PASS rejected kernel' >> "$out/controls.txt"
mkdir "$tmp/modules"
cp ../nist_cover_separator_20261115/data/modules-*.json "$tmp/modules/"
for kind in count frobenius form missing duplicate; do
    "$tmp/make-control" "$kind" ../nist_cover_separator_20261115/data/modules-192.json "$tmp/modules/modules-192.json"
    if "$tmp/module-audit" "$tmp/modules" "$tmp/unused-kernels" > "$out/control-$kind.stdout" 2> "$out/control-$kind.stderr"; then
        echo "ERROR accepted control $kind" >&2; exit 1
    fi
    printf '%s\n' "PASS rejected $kind" >> "$out/controls.txt"
done
for kind in point pairing curve; do
    case "$kind" in
        point) src=inputs/p192-d7-prym.json ;;
        pairing) src=inputs/p192-d7-pairings.json ;;
        curve) src=inputs/p192-d7-prym.json ;;
    esac
    "$tmp/make-control" "$kind" "$src" "$tmp/bad-$kind.json"
    if [ "$kind" = pairing ]; then
        if "$tmp/point-replay" inputs/p192-d7-prym.json "$tmp/bad-$kind.json" inputs/p192-elliptic.json > "$out/control-$kind.stdout" 2> "$out/control-$kind.stderr"; then echo 'ERROR accepted pairing' >&2; exit 1; fi
    elif [ "$kind" = point ]; then
        if "$tmp/point-replay" "$tmp/bad-$kind.json" inputs/p192-d7-pairings.json inputs/p192-elliptic.json > "$out/control-$kind.stdout" 2> "$out/control-$kind.stderr"; then echo 'ERROR accepted point' >&2; exit 1; fi
    else
        if "$tmp/cover-replay" "$tmp/bad-$kind.json" > "$out/control-$kind.stdout" 2> "$out/control-$kind.stderr"; then echo 'ERROR accepted curve' >&2; exit 1; fi
    fi
    printf '%s\n' "PASS rejected $kind" >> "$out/controls.txt"
done
sha256sum *.cpp json.hpp inputs/*.json > "$out/post-call-source-manifest.sha256"
cat "$out/module-summary.txt" "$out/point-replay.txt" "$out/cover-replay.txt" "$out/actual-cover-torsion-summary.txt" "$out/controls.txt"
