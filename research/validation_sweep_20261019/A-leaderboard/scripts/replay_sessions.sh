#!/bin/sh
# Seeded independent replay for ecbench-session rows: for each row, 3 verified measured records of the named
# arm are drawn with scripts/sample.sh (seed from REPLAY_SEED.md), and the recorded scalar is checked
# against the record's own curve, generator and public target by certverify (no repository code).
# Rows whose curve parameters are not in the record (random binary curves are generated from a seed) are
# emitted as NOT_COVERED, never skipped.
# usage: replay_sessions.sh <certverify-binary>   (repo root; read-only)
CV="$1"; here=research/validation_sweep_20261019/A-leaderboard/scripts
row() { # row_id session_dir arm_regex
  id="$1"; s="$2"; arm="$3"
  ids=$(jq -r --arg arm "$arm" 'select((.warmup|not) and (.outcome.status=="verified") and (.arm|test($arm))) | .record_id' "$s/records.jsonl")
  [ -n "$ids" ] || { printf '%s\tNOT_COVERED\tno verified measured record for arm %s\n' "$id" "$arm"; return; }
  for rid in $(echo "$ids" | $here/sample.sh "$id" 3); do
    jq -r --arg id "$id/$rid" 'select(.record_id=="'"$rid"'") |
      .workload as $w | $w.curve as $c |
      if $w.curve_spec.kind=="koblitz" then
        [$id,"gf2m?",($w.curve_spec.n|tostring),($w.curve_spec.a|tostring),"1",$c.generator[0],$c.generator[1],$w.target[0],$w.target[1],.outcome.recovered,($c.r|tostring)]|@tsv
      elif $w.curve_spec.kind=="prime_explicit" then
        [$id,"fp",($w.curve_spec.p|tostring),($w.curve_spec.a|tostring),($w.curve_spec.b|tostring),$c.generator[0],$c.generator[1],$w.target[0],$w.target[1],.outcome.recovered,($c.r|tostring)]|@tsv
      else [$id,"NOT_COVERED","curve_spec "+$w.curve_spec.kind+" has no explicit coefficients in the record"]|@tsv end' "$s/records.jsonl"
  done
}
R=research
{
row n37-bounded:ic-k16        $R/ecbench_n37_rank_columns_20261004/sessions/mac_arm64_l0_01 '^ic-k16$'
row n37-k8k16:ic-k8           $R/ecbench_n37_k8_k16_20261004/sessions/mac_arm64_l0_01 '^ic-k8$'
row n37-online-ir:ic-k16      $R/ecbench_n37_online_ir_20261004/sessions/mac_arm64_l0_01 '^ic-k16$'
row n37-native-wall:ic-k16    $R/ecbench_n37_native_online_wall_20261004/sessions/hosted_ubuntu_01 '^ic-k16$'
row n41:ic-k16                $R/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0-02 '^ic-k16$'
row n41:rho-strong            $R/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0-02 '^rho-strong$'
row n53:ic-k16                $R/ecbench_n41_n53_shared_rank_20261005/sessions/mac-l0-02 '^ic-k16$'
row f6ladder-m13:ic-f4        $R/f6_ic_ecbench_ladder_20261005/sessions/n13 '^ic-f4$'
row f6ladder-m13:ic-f6        $R/f6_ic_ecbench_ladder_20261005/sessions/n13 '^ic-f6$'
row f6ladder-m19:ic-f4        $R/f6_ic_ecbench_ladder_20261005/sessions/n19 '^ic-f4$'
row f6ladder-m19:ic-f6        $R/f6_ic_ecbench_ladder_20261005/sessions/n19 '^ic-f6$'
row f6ladder-m23:ic-f4        $R/f6_ic_ecbench_ladder_20261005/sessions/n23 '^ic-f4$'
row f6ladder-m23:ic-f6        $R/f6_ic_ecbench_ladder_20261005/sessions/n23 '^ic-f6$'
row xfam-K13:inherited-f4     $R/f4_f5_f6_cross_family_20261011/sessions/K13 '^inherited-f4$'
row xfam-K13:f6-ic            $R/f4_f5_f6_cross_family_20261011/sessions/K13 '^f6-ic$'
row xfam-K19:inherited-f4     $R/f4_f5_f6_cross_family_20261011/sessions/K19 '^inherited-f4$'
row xfam-K19:f6-ic            $R/f4_f5_f6_cross_family_20261011/sessions/K19 '^f6-ic$'
row xfam-R13:inherited-f4     $R/f4_f5_f6_cross_family_20261011/sessions/R13 '^inherited-f4$'
row xfam-R19:inherited-f4     $R/f4_f5_f6_cross_family_20261011/sessions/R19 '^inherited-f4$'
row conductor-8209:ic-mitm    $R/ecbench_conductor_gap_20261012/sessions/ell8209-q29 '^ic-mitm$'
row conductor-131101:ic-mitm  $R/ecbench_conductor_gap_20261012/sessions/ell131101-q37 '^ic-mitm$'
} > /tmp/replay_sessions.cases.$$
grep -v NOT_COVERED /tmp/replay_sessions.cases.$$ | "$CV"
grep NOT_COVERED /tmp/replay_sessions.cases.$$
rm -f /tmp/replay_sessions.cases.$$
