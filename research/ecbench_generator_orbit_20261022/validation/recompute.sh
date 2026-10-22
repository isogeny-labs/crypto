#!/usr/bin/env bash
# Validator recompute driver (independent of ecbench band / tables/).
# Usage: bash research/ecbench_generator_orbit_20261022/validation/recompute.sh
# Writes recompute-<session>.json, h1-<session>.json and recompute-summary.json
# next to this script.  Reads raw sessions/<session>/records.jsonl,
# docs/ic/calibration.json, claimed-3dec.json, and (only as the `claimed`
# side of each headline) tables/band-<session>.json.
set -euo pipefail
V="$(cd "$(dirname "$0")" && pwd)"
HERE="$(dirname "$V")"
ROOT="$(git -C "$HERE" rev-parse --show-toplevel)"
CAL="$ROOT/docs/ic/calibration.json"
SESSIONS="a1-n29 a0-n13 a1-n17 a0-n19 a0-n23 a0-n31 a0-n37"
for s in $SESSIONS; do
  jq -s --slurpfile cal "$CAL" --arg session "$s" -f "$V/band_recompute.jq" \
    "$HERE/sessions/$s/records.jsonl" > "$V/recompute-$s.json"
  jq -s --arg session "$s" -f "$V/h1_check.jq" \
    "$HERE/sessions/$s/records.jsonl" > "$V/h1-$s.json"
  # diagnostic only (findings F2-F4), not a headline
  jq -s --slurpfile fp "$ROOT/src/cryptanalysis/ecbench/full_prices.json" --arg session "$s" \
    -f "$V/alike_band.jq" "$HERE/sessions/$s/records.jsonl" > "$V/alike-$s.json"
done
args=()
for s in $SESSIONS; do
  args+=(--slurpfile "rc_${s//-/_}" "$V/recompute-$s.json"
         --slurpfile "band_${s//-/_}" "$HERE/tables/band-$s.json"
         --slurpfile "h1_${s//-/_}" "$V/h1-$s.json")
done
jq -n --slurpfile claimed "$V/claimed-3dec.json" --arg sessions "$SESSIONS" "${args[@]}" '
  def tol: 1e-6;
  def agree($c; $r): (($r - $c) | fabs) <= tol * ($c | fabs);
  def r3: (. * 1000 | round) / 1000;
  ($claimed[0]) as $cl
  | ($sessions | split(" ")) as $ss
  | [ $ss[] as $s
      | ($s | gsub("-"; "_")) as $k
      | ($ARGS.named["rc_" + $k][0]) as $rc
      | ($ARGS.named["band_" + $k][0]) as $band
      | $cl.mean_s[$s] | to_entries[] as $e
      | ($rc.arms[] | select(.arm == $e.key)) as $mine
      | ($band.rows[] | select(.arm == $e.key)) as $theirs
      | range(0; 3) as $i
      | ($theirs.mean_s[$i]) as $c
      | ($mine.mean_s[$i]) as $r
      | { name: "mean_S[\($s)/\($e.key)/w=\([0, 0.1, 1][$i])]",
          claimed: $c, recomputed: $r, rel_tol: tol,
          agrees: agree($c; $r),
          rel_diff: (if $c == 0 then null else (($r - $c) / $c) end),
          claimed_3dec: $e.value[$i],
          recomputed_3dec: ($r | r3),
          agrees_3dec: (($r | r3) == $e.value[$i]),
          records: $mine.n } ] as $band_rows
  | [ $ss[] as $s | $ARGS.named["h1_" + ($s | gsub("-"; "_"))][0] ] as $h1
  | ($h1 | map(.pairs) | add) as $pairs
  | ($h1 | map(.pairs_equal) | add) as $equal
  | ($h1 | map(select(.session == "a1-n29"))[0].gen_mean_trials_measured) as $trials
  | { schema: "validator-recompute-summary/v1",
      band_headlines: $band_rows,
      band_all_agree: ($band_rows | map(.agrees) | all),
      band_all_agree_3dec: ($band_rows | map(.agrees_3dec) | all),
      h1: { per_session: ($h1 | map({session, lazy_records, layout_records, pairs, pairs_with_partner, pairs_equal,
                                     lazy_keys_unique, layout_keys_unique, layout_without_lazy_partner,
                                     null_fields_in_signature, phase_names_ok, disagreements})),
            pairs_total: $pairs, pairs_equal_total: $equal,
            all_partnered: ($h1 | map(.pairs == .pairs_with_partner and .layout_without_lazy_partner == 0
                                      and .lazy_records == .layout_records) | all),
            claimed_pairs_total: $cl.h1_pairs_total },
      h1_headlines: [
        { name: "H1 lazy/layout (workload, round, warmup) pairs, all sessions", claimed: $cl.h1_pairs_total,
          recomputed: $pairs, rel_tol: tol, agrees: agree($cl.h1_pairs_total; $pairs) },
        { name: "H1 lazy/layout pairs equal on trials, relations, rank, recovered, pair_table_entries", claimed: $cl.h1_pairs_total,
          recomputed: $equal, rel_tol: tol, agrees: agree($cl.h1_pairs_total; $equal) } ],
      n29_headline: { name: "a1-n29 ic-gen-layout-c1 mean phases[2].native.trials (non-warm-up)",
                      claimed: $cl.n29_gen_layout_mean_trials, recomputed: $trials, rel_tol: tol,
                      agrees: agree($cl.n29_gen_layout_mean_trials; $trials) } }
' > "$V/recompute-summary.json"
jq -r '"band agree: \(.band_all_agree) (3dec: \(.band_all_agree_3dec)); H1 pairs \(.h1.pairs_total) equal \(.h1.pairs_equal_total); n29 trials \(.n29_headline.recomputed)"' "$V/recompute-summary.json"
