#!/bin/sh
# Census of the FIPS 186-2 binary-curve seed runs: enumerate candidates
# (Rust), convert from the normal basis and point-count (GP). Thin
# orchestration only.
#   research/nist_binary_skipped_20261020/census.sh
set -eu
here=$(cd "$(dirname "$0")" && pwd)
out="$here/census"
mkdir -p "$out"
cargo run --quiet --release --example nist_binary_seed_candidates -- --published > "$out/published_nb.txt"
cargo run --quiet --release --example koblitz_degree_sweep -- --from 100 --to 600 > "$out/koblitz_sweep.txt"
# name run blocks poly
runs='B-163 0 0 z^163+z^7+z^6+z^3+1
B-233 446413329 155 z^233+z^74+1
B-283 446413432 32 z^283+z^12+z^7+z^5+1
B-409 446413509 60 z^409+z^87+1
B-571 446413564 73 z^571+z^10+z^5+z^2+1'
echo "$runs" | while read -r name run blocks poly; do
  set -- $(grep "^$name " "$out/published_nb.txt")
  m=$2; cnb=$4
  bpub=$(grep -A0 "\"$name\"" "$here/../../src/cryptanalysis/nist_seed_provenance.rs" | sed -n 's/.*b_hex: "\([0-9a-f]*\)".*/\1/p')
  cand=""
  if [ "$run" != 0 ]; then
    cand="$out/candidates_$run.txt"
    cargo run --quiet --release --example nist_binary_seed_candidates -- --run "$run" --blocks "$blocks" > "$cand"
  fi
  printf 'M=%s;\nFPOLY=%s;\nCNB_PUB=0x%s;\nB_PUB=0x%s;\nCAND="%s";\nNAME="%s";\n' \
    "$m" "$(echo "$poly" | sed "s/z/'z/g")" "$cnb" "$bpub" "$cand" "$name" \
    | cat - "$here/census.gp" | gp -q 2>/dev/null | grep -E "^(calib|cand) " > "$out/orders_$name.txt"
  echo "$name done"
done
# Summary: per run, rows, accepted (n prime and MOV), index of each accepted.
{
  printf '{\n  "runs": [\n'
  first=1
  for name in B-233 B-283 B-409 B-571; do
    f="$out/orders_$name.txt"
    [ $first = 1 ] || printf ',\n'; first=0
    awk -v name="$name" '
      $1=="calib" {conj=$3; ord=$4; hits=$5}
      $1=="cand"  {rows++; last=$2; if ($8==1) ok++; if ($6==1 && $7==1) acc = acc (acc==""?"":",") $2}
      END {printf "    {\"curve\": \"%s\", \"candidates\": %d, \"checks_pass\": %d, \"accepted\": [%s], \"published\": %d, \"calibration\": {\"conjugate\": %d, \"bit_order\": %d, \"matches\": %d}}", name, rows, ok, acc, last, conj, ord, hits}' "$f"
  done
  printf '\n  ],\n'
  awk '$1=="calib" {printf "  \"B-163_calibration\": {\"conjugate\": %d, \"bit_order\": %d, \"matches\": %d}\n", $3, $4, $5}' "$out/orders_B-163.txt"
  printf '}\n'
} > "$out/summary.json"
