#!/usr/bin/env bash
# Validator recompute driver: rebuild every intermediate the validator report
# uses from committed raw data only (runs/extract, runs/uncharged,
# runs/slugmap, census/V*/vertices.jsonl, runs/progress.jsonl).
# No producer code is read or run.
#
# Usage: recompute.sh STUDY WORK [--no-bootstrap]
# Then:  replay_sample.sh / unc_sample.sh (replays), build_validation.sh, write_md.sh
set -euo pipefail
STUDY=$1; W=$2; BOOT=${3:-}
HERE=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$W"
cd "$STUDY"

# 1. slug -> (a, b, p, r, G) per spec; census vertices with their j; slug -> vertex by j-invariant
for f in runs/slugmap/*.json; do s=$(basename "$f" .json)
  jq -r --arg s "$s" 'to_entries[] | [$s, .key, .value.a, .value.b, .value.p, .value.r, .value.gx, .value.gy] | @tsv' "$f"
done > "$W/slugs.tsv"
for v in V1 V2 V3; do jq -r '[.volcano, .id, .level, .a, .b, .p, .j] | @tsv' "census/$v/vertices.jsonl"; done |
  awk -v mode=census -f "$HERE/jmap.awk" > "$W/vertex_j.tsv"
cut -f1-5 "$W/slugs.tsv" | awk -v mode=slug -f "$HERE/jmap.awk" > "$W/slug_j.tsv"
awk -F'\t' 'NR == FNR { vx[$1 "|" $2] = $3 "\t" $4; next }
  { k = $3 "|" $4; if (k in vx) print $1 "\t" $2 "\t" $3 "\t" vx[k]; else { miss++; print "MISS", $0 > "/dev/stderr" } }
  END { print "unmapped slugs: " miss + 0 > "/dev/stderr" }' "$W/vertex_j.tsv" "$W/slug_j.tsv" > "$W/slug_vertex.tsv"

# 2. measured rows joined with vertex and uncharged counters
awk -v OFS='\t' -f "$HERE/join.awk" "$W/slug_vertex.tsv" \
  <(for f in runs/uncharged/*.csv.xz; do xz -dc "$f" | tail -n +2; done) \
  <(for f in runs/extract/*.csv.xz; do xz -dc "$f" | tail -n +2; done) > "$W/measured.tsv" 2> "$W/join_stats.txt"

# 3. point estimates, counts, curve effect, GAE band
awk -f "$HERE/point_estimates.awk" "$W/measured.tsv" > "$W/pe.tsv"
awk -F'\t' '{ if ($14 == "true") v++; k = $1 "|" $2 "|" $8; n[k]++; if ($14 == "true") vv[k]++; r[$1]++ }
  END { min = 1e18; for (k in n) if (vv[k] == n[k] && n[k] < min) min = n[k]
        printf "count.verified\t%d\ncp95_lower_smallest\t%.17g\ncount.cells\t%d\n", v, exp(log(0.025) / min), length(n)
        for (x in r) printf "count.rows.%s\t%d\n", x, r[x] }' "$W/measured.tsv" >> "$W/pe.tsv"
printf 'count.sessions\t%d\n' "$(jq -r 'select(.status == "done" and .audit_ok == true) | .spec' runs/progress.jsonl | sort -u | wc -l)" >> "$W/pe.tsv"
for v in V1 V2; do awk -v V=$v -f "$HERE/curve_effect.awk" "$W/measured.tsv"; done | sort > "$W/curve_effect.tsv"
# V3 crater IC mean S and SE (ddof 1) per curve model (results.json crater_ic_by_model, erratum A3)
awk -F'\t' '$14 == "true" && $1 == "V3" && $3 == 0 && $8 == "ic" { s[$5] += $11; q[$5] += $11 * $11; n[$5]++ }
  END { for (m in s) { mu = s[m] / n[m]; v = (q[m] - n[m] * mu * mu) / (n[m] - 1); printf "cm.%s.mean\t%.17g\ncm.%s.se\t%.17g\ncm.%s.n\t%d\n", m, mu, m, sqrt(v / n[m]), m, n[m] } }' "$W/measured.tsv" >> "$W/pe.tsv"
awk -F'\t' 'BEGIN { P[1] = 0; P[2] = 0.1; P[3] = 1 }
  $14 == "true" && $2 != "ctl" && ($8 == "ic" || $8 == "rho-neg") { for (i = 1; i <= 3; i++) { ps = $11 * ($16 + P[i] * $18) / $16; s[$1, $8, i] += ps; n[$1, $8, i]++ } }
  $14 == "true" && $1 == "V3" && $3 == 0 && $2 != "ctl" && ($8 == "ic" || $8 == "rho-neg" || $8 == "rho-cubic") { for (i = 1; i <= 3; i++) { ps = $11 * ($16 + P[i] * $18) / $16; c[$8, i] += ps; cn[$8, i]++ } }
  END { split("V1 V2 V3", VV, " ")
    for (v = 1; v <= 3; v++) for (i = 1; i <= 3; i++) printf "band\t%s\t%s\tic=%.17g\trho_neg=%.17g\tratio=%.17g\tn=%d\n", VV[v], P[i], s[VV[v], "ic", i] / n[VV[v], "ic", i], s[VV[v], "rho-neg", i] / n[VV[v], "rho-neg", i], (s[VV[v], "ic", i] / n[VV[v], "ic", i]) / (s[VV[v], "rho-neg", i] / n[VV[v], "rho-neg", i]), n[VV[v], "ic", i]
    for (i = 1; i <= 3; i++) printf "v3crater\t%s\tic=%.17g\trho_cubic=%.17g\trho_neg=%.17g\tic_over_cubic=%.17g\tn=%d\n", P[i], c["ic", i] / cn["ic", i], c["rho-cubic", i] / cn["rho-cubic", i], c["rho-neg", i] / cn["rho-neg", i], (c["ic", i] / cn["ic", i]) / (c["rho-cubic", i] / cn["rho-cubic", i]), cn["rho-cubic", i] }' "$W/measured.tsv" > "$W/band.tsv"

# 4. REPORT.md table cells against results.json
"$HERE/report_tables_check.sh" "$STUDY" > "$W/report_tables.txt"

[ "$BOOT" = "--no-bootstrap" ] && exit 0

# 5. bootstrap inputs (stratum, cluster, a, b, c, d) and runs; seeds 777001-777006
M=$W/measured.tsv
awk -F'\t' -v OFS='\t' '$14 == "true" && $1 == "V3" && $3 == 0 && ($2 == "D" || $2 == "N" || $2 == "ctl") && ($8 == "rho-cubic" || $8 == "rho-neg") {
  k = $2 "|" $6 "|" $7 "|" $9; if ($8 == "rho-cubic") c[k] = $11; else n[k] = $11 } END { for (k in c) if (k in n) print 1, k, c[k], 1, n[k], 1 }' "$M" > "$W/boot_pc.tsv"
awk -F'\t' -v OFS='\t' '$14 == "true" && $1 == "V1" && $8 == "rho-plain" && $2 != "ctl" && ($3 == 0 || $3 == 1) {
  if ($3 == 1) print 1, $4, $11, 1, 0, 0; else print 0, $4, 0, 0, $11, 1 }' "$M" > "$W/boot_v1rp_L1.tsv"
awk -F'\t' -v OFS='\t' '$14 == "true" && $1 == "V1" && $2 != "ctl" && ($8 == "ic" || $8 == "rho-neg") {
  k = $3 "\t" $4 "\t" $2 "|" $6 "|" $7 "|" $9; if ($8 == "ic") a[k] = $11; else b[k] = $11 }
  END { for (k in a) if (k in b) { split(k, z, "\t"); print z[1], z[2], a[k], 1, b[k], 1 } }' "$M" > "$W/boot_v1icrho.tsv"
awk -F'\t' -v OFS='\t' '$14 == "true" && $1 == "V1" && $8 == "rho-plain" && ($2 == "D" || $2 == "S") {
  k = $3 "\t" $4 "\t" $7 "|" $9; if ($2 == "S") a[k] = $11; else b[k] = $11 }
  END { for (k in a) if (k in b) { split(k, z, "\t"); print z[1], z[2], a[k], 1, b[k], 1; print z[1], z[2] "|" z[3], a[k], 1, b[k], 1 > "/dev/stderr" } }' "$M" > "$W/boot_pair_vx.tsv" 2> "$W/boot_pair_tr.tsv"
awk -F'\t' -v OFS='\t' '$14 == "true" && $1 == "V3" && $3 == 0 && ($2 == "D" || $2 == "N") && ($8 == "ic" || $8 == "rho-cubic") {
  k = $2 "|" $6 "|" $7 "|" $9; if ($8 == "ic") a[k] = $11; else b[k] = $11 } END { for (k in a) if (k in b) print 1, k, a[k], 1, b[k], 1 }' "$M" > "$W/boot_v3c.tsv"
# seeds 777001-777006 in this order; the six runs are independent, so they run in parallel
seed=777001
for b in pc v1rp_L1 v1icrho pair_vx pair_tr v3c; do
  awk -v seed=$seed -v reps=10000 -f "$HERE/cluster_boot.awk" "$W/boot_$b.tsv" > "$W/rep_$b.txt" & seed=$((seed + 1))
done
wait
for b in pc v1rp_L1 v1icrho pair_vx pair_tr v3c; do
  "$HERE/quantiles.sh" "$W/rep_$b.txt" | awk -v f=$b -F'\t' '{ printf "%s%s\"%s\":%s", (NR == 1 ? "{\"check\":\"" f "\"," : ""), (NR == 1 ? "" : ","), $1, $2 } END { print "}" }'
done > "$W/boot_q.jsonl"
