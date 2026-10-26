#!/usr/bin/env bash
# R10 ledger pin: reproduce every DERIVED number in README.md from the
# committed raw records. Thin orchestration only (jq + awk over recorded
# values); nothing is re-run and no new measurement is made.
#
#   research/ic_tool_program/ledger/r10-20261019/rerate.sh
#
# Writes data/genus3_rows.tsv, data/genus3_derived.txt and
# data/koblitz_derived.txt beside this script.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
r="$(cd "$here/../../.." && pwd)"   # research/
mkdir -p "$here/data"

# ---------------------------------------------------------------- genus 3
# One row per verified solve on test curves 0-2 (the shared instance chain:
# q = next prime above 2^b, seed 20261014; each round's gate 0 reproduces
# its predecessor's incumbent bit for bit). The cx suffix (@8/@16) is the
# frozen per-size SGE setting of one arm, so it is folded into the arm name.
emit() { # round dir
  find "$r/$2" -name '*.jsonl' -print0 | sort -z | xargs -0 cat |
    jq -r --arg rd "$1" '. as $x | select(.curve <= 2) | (.results // [])[]
      | select(.correct == true)
      | [$rd, $x.bits, $x.curve, (.arm | sub("@[0-9]+"; "")), .cost,
         (.rel_cost // "NA"), (.la_cost // "NA")] | @tsv'
}
{
  printf 'round\tb\tcurve\tarm\tcost\trel_cost\tla_cost\n'
  emit r1  ic_improve_20261014/04-genus3-2lp/results/run-20261014
  emit r2a ic_improve_20261015/01-genus3-linalg/results/run
  emit r2b ic_improve_20261015/06-genus3-kappa-bw/results/run
  emit r3  ic_improve3_20261015/01-genus3-relations/results/run
  emit r4  ic_improve4_20261015/01-genus3-relations/results/run
} > "$here/data/genus3_rows.tsv"

awk -F'\t' '
NR == 1 { next }
$4 ~ /^aa/ { next }                               # A/A streams are not arms
{
  k = $1 ":" $4; b = $2; c = $3; v = log($5) / log(2)
  arms[k] = 1; L[k, b, c] += v; nL[k, b, c]++; n[k, b]++; s[k, b] += v
  if ($6 != "NA") { R[k, b] += log($6) / log(2); nr[k, b]++ }
  if ($7 != "NA") { A[k, b] += log($7) / log(2); na[k, b]++ }
  # every solve is a point in the exponent fit (log2 cost against b)
  F[k, ++m[k]] = b SUBSEP v
}
function fit(k, lo, hi,   i, p, x, y, sx, sy, sxx, sxy, N) {
  for (i = 1; i <= m[k]; i++) {
    split(F[k, i], p, SUBSEP); x = p[1]; y = p[2]
    if (x < lo || x > hi) continue
    N++; sx += x; sy += y; sxx += x * x; sxy += x * y
  }
  if (N < 3 || sxx * N == sx * sx) return "NA"
  return sprintf("%.4f", (N * sxy - sx * sy) / (N * sxx - sx * sx))
}
function gm(k, b) { return n[k, b] ? sprintf("%.4e", 2 ^ (s[k, b] / n[k, b])) : "NA" }
function pair(a, z, b,   c, t, N) {               # geomean over curves of a/z
  for (c = 0; c <= 2; c++) if ((a, b, c) in L && (z, b, c) in L) { t += L[a, b, c] / nL[a, b, c] - L[z, b, c] / nL[z, b, c]; N++ }
  return N ? sprintf("%.4f", 2 ^ (t / N)) : "NA"
}
END {
  print "## per-size geometric mean over curves 0-2 (whole cost; relation; LA)"
  for (k in arms) for (b = 8; b <= 22; b++) if (n[k, b])
    printf "gm\t%s\t%d\t%s\t%s\t%s\n", k, b, gm(k, b),
      (nr[k, b] ? sprintf("%.4e", 2 ^ (R[k, b] / nr[k, b])) : "NA"),
      (na[k, b] ? sprintf("%.4e", 2 ^ (A[k, b] / na[k, b])) : "NA")
  print "## fitted exponent d log2(cost) / d b, every solve a point"
  for (k in arms) printf "fit\t%s\t10..18 %s\t10..20 %s\t12..21 %s\t12..22 %s\t16..21 %s\t16..22 %s\n", k,
    fit(k, 10, 18), fit(k, 10, 20), fit(k, 12, 21), fit(k, 12, 22), fit(k, 16, 21), fit(k, 16, 22)
  print "## paired ratios (geomean over curves 0-2 of per-curve cost ratio)"
  TL = "r2a:tuned:lpsgewd"; C4 = "r4:c4:lpsgebw2o100c+C1C2"
  for (b = 10; b <= 21; b++) printf "pair\tr1:ic2lp/r1:ic1lp\t%d\t%s\n", b, pair("r1:ic2lp", "r1:ic1lp", b)
  for (b = 12; b <= 21; b++) {
    printf "pair\tr2b:main:lpsgebw2/TL\t%d\t%s\n", b, pair("r2b:main:lpsgebw2", TL, b)
    printf "pair\tr2b:sched:lpsgebw2/TL\t%d\t%s\n", b, pair("r2b:sched:lpsgebw2", TL, b)
    printf "pair\tr2b:sched:lpsgewd/TL\t%d\t%s\n", b, pair("r2b:sched:lpsgewd", TL, b)
    printf "pair\tr3:r2:lpsgebw2+R1/TL\t%d\t%s\n", b, pair("r3:r2:lpsgebw2+R1", TL, b)
    printf "pair\tC4/TL\t%d\t%s\n", b, pair(C4, TL, b)
    printf "pair\tC4/r1:ic2lp\t%d\t%s\n", b, pair(C4, "r1:ic2lp", b)
    printf "pair\tr4:r2:lpsgebw2+C1/r3:sched:lpsgebw2+R1\t%d\t%s\n", b, pair("r4:r2:lpsgebw2+C1", "r3:sched:lpsgebw2+R1", b)
  }
  for (b = 12; b <= 22; b++) printf "pair\tC4/r4:r2:lpsgebw2\t%d\t%s\n", b, pair(C4, "r4:r2:lpsgebw2", b)
  print "## E0 rho on the chain (round 1 rho arm, negation map, 64 multipliers)"
  for (b = 10; b <= 20; b++) printf "rho\t%d\t%s\tC4/rho %s\n", b, gm("r1:rho", b), pair(C4, "r1:rho", b)
}' "$here/data/genus3_rows.tsv" | sort > "$here/data/genus3_derived.txt"

# --------------------------------------------- Koblitz compact orbit (A)
# Held-out test rows of ic_improve round 2 item 02 (seeds 1-16 / 1-12, 8
# targets each): mean counted cost (S3 root solves per complete solve),
# mean wall ms, and the setup / online split.
awk -F, 'NR > 1 && $5 == "test" {
  k = $1 " " $4 "@" $2; c[k] += $10; w[k] += $15
  t[k] += $8; wo[k] += $14; N[k]++
} END {
  for (k in N) printf "kob\t%s\tseeds %d\tcounted %.0f\tsetup_counted %.0f\tonline_counted %.0f\twall_ms %.1f\tsetup_ms %.1f\tonline_ms %.2f\n",
    k, N[k], c[k] / N[k], (c[k] - t[k]) / N[k], t[k] / N[k], w[k] / N[k], (w[k] - wo[k]) / N[k], wo[k] / N[k]
}' "$r/ic_improve_20261015/02-freerel-production/data/rows.csv" | sort > "$here/data/koblitz_derived.txt"

# The same rows in the ledger's unit, S = wall / (unit_ns × √r), with v0's
# pinned unit_ns for the same ICV1 curves (construct:41/53/61:0 are k0n41,
# k0n53, k0n61: r and log2 r agree). Different host, so cross-host.
jq -r '.baselines[0].sizes[] | select(.size == "k0n41" or .size == "k0n53" or .size == "k0n61")
  | [(.size | ltrimstr("k0n")), .curve_alias, .log2_r, .unit_ns] | @tsv' "$r/ic_tool_program/baselines.json" |
while IFS=$'\t' read -r n alias lr u; do
  awk -v n="$n" -v a="$alias" -v lr="$lr" -v u="$u" -F'\t' '$2 ~ "^" n " " {
    split($7, w, " "); split($8, s, " "); split($9, o, " ")
    d = u * 2 ^ (lr / 2) / 1e6                       # ns × √r, in ms
    printf "kobS\t%s\t%s\ts_cold %.3f\ts_setup %.3f\ts_online %.4f\n", $2, a, w[2] / d, s[2] / d, o[2] / d
  }' "$here/data/koblitz_derived.txt"
done >> "$here/data/koblitz_derived.txt"

# ------------------------------------------------------- k = 4, D4 family
# ic_improve round 2 item 04 test rows: per p, geometric mean over its three
# curves of S (charged GAE / √r) for each arm, and of the negation-rho S.
jq -r '.rows[] | . as $x | (.arms | to_entries[]) | [$x.p, $x.seed, .key, .value.S_charged, $x.rho_neg_S_reference] | @tsv' \
  "$r/ic_improve_20261015/04-d4-2lp-fix/raw/summary.json" |
awk -F'\t' '{ k = $1 SUBSEP $3; s[k] += log($4); n[k]++; P[$1] = 1; A[$3] = 1
  L[$1, $2, $3] = log($4); if (!(($1, $2) in R)) { R[$1, $2] = 1; rh[$1] += log($5); nr[$1]++ } }
END {
  for (p in P) {
    printf "d4\tp=%d\trho_neg_S %.4f", p, exp(rh[p] / nr[p])
    for (a in A) if (n[p, a]) printf "\t%s %.4e", a, exp(s[p, a] / n[p, a])
    printf "\td4-ed/rho %.4e\n", exp(s[p, "d4-ed"] / n[p, "d4-ed"] - rh[p] / nr[p])
  }
  for (a in A) { t = 0; N = 0
    for (x in L) { split(x, q, SUBSEP); if (q[3] != a || !((q[1], q[2], "d4-ed") in L)) continue
      t += L[x] - L[q[1], q[2], "d4-ed"]; N++ }
    if (N) printf "d4pair\t%s/d4-ed\tcurves %d\t%.4f\n", a, N, exp(t / N) }
}' | sort -t$'\t' -k1,1 -k2,2V > "$here/data/d4_derived.txt"

echo "wrote $here/data/{genus3_rows.tsv,genus3_derived.txt,koblitz_derived.txt,d4_derived.txt}"
