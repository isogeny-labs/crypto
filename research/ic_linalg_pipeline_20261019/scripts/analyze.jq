# Derived figures for PROTOCOL.md "Measures", from one ecbench records.jsonl
# (jq -s). Every value is recomputed from recorded fields; nothing is measured here.
#
# Pricing band (AGENTS.md §5a): S_w = (total_gae + w * U) / sqrt(r), where U sums
# the run's unpriced counters: for ic.pipeline the native counts behind
# legendre_symbols, sqrt_solves, lookups, row_ops and target_guard_probes
# (summed over phases); for rho.negation counters.canonicalisations_uncharged.
def uncharged:
  if .method.id == "rho.negation" then (.counters.canonicalisations_uncharged // 0)
  else [ .phases[].native // {} | to_entries[]
         | select(.key | IN("legendre_symbols","sqrt_solves","lookups","row_ops","target_guard_probes"))
         | .value ] | add // 0 end;
def sqrt_r: (.workload.curve.r | tonumber | sqrt);
def s_w($w): (.cost.total_gae + $w * uncharged) / sqrt_r;
def la(f): [ .phases[] | select(.name == "linear_algebra") | f ] | first;
def key: "\(.workload.workload_id)/\(.round)";
def stats: sort as $s | { n: length, min: $s[0], median: $s[(length/2|floor)], max: $s[-1] };
def ratio_sum($a; $b; f): ([ $b[] | f ] | add) / ([ $a[] | f ] | add);

[ .[] | select(.warmup == false) ] as $all
| ($all | map(select(.outcome.status != "verified")) | length) as $unverified
| ($all | map({ (key + "|" + .arm): . }) | add) as $by
| ($all | map(.arm) | unique) as $arms
| ($all | map(key) | unique) as $keys
| def pairs($a; $b): [ $keys[] as $k | select($by[$k+"|"+$a] and $by[$k+"|"+$b]) | [ $by[$k+"|"+$a], $by[$k+"|"+$b] ] ];
  def cmp($a; $b):
    pairs($a; $b) as $p
    | ($p | map(.[0])) as $A | ($p | map(.[1])) as $B
    | { a: $a, b: $b, pairs: ($p | length),
        S_ratio: { w0: ratio_sum($A; $B; s_w(0)), w0_1: ratio_sum($A; $B; s_w(0.1)), w1: ratio_sum($A; $B; s_w(1)) },
        S_pair_ratio_w1: ($p | map((.[1] | s_w(1)) / (.[0] | s_w(1))) | stats) };
  def lacmp($a; $b):
    pairs($a; $b) as $p
    | ($p | map(.[0])) as $A | ($p | map(.[1])) as $B
    | { a: $a, b: $b, pairs: ($p | length),
        la_wall_ratio_sum: ratio_sum($A; $B; la(.wall_ns)),
        la_wall_pair_ratio: ($p | map((.[1] | la(.wall_ns)) / (.[0] | la(.wall_ns))) | stats),
        row_ops_ratio_sum: ratio_sum($A; $B; la(.native.row_ops)) };
  def arm_summary($arm):
    [ $all[] | select(.arm == $arm) ] as $r
    | { arm: $arm, runs: ($r | length),
        verified: ($r | map(select(.outcome.status == "verified")) | length),
        levels: ($r | map(.isolation.level) | group_by(.) | map({ (.[0]): length }) | add),
        S_w0_mean: ($r | map(s_w(0)) | add / length),
        S_w1_mean: ($r | map(s_w(1)) | add / length),
        uncharged_mean: ($r | map(uncharged) | add / length) }
      + (if $r[0].method.id == "ic.pipeline" then
          { linalg: $r[0].method.params.linalg, factor_base: $r[0].method.params.factor_base,
            row_ops: ($r | map(la(.native.row_ops)) | stats),
            rows: ($r | map(la(.native.rows)) | stats),
            relations_trials: ($r | map([.phases[] | select(.name=="relations") | .native.trials] | first) | stats),
            wall_ms_mean: { oracle_setup: ($r | map([.phases[] | select(.name=="oracle_setup") | .wall_ns] | first) | add / length / 1e6),
                            relations: ($r | map([.phases[] | select(.name=="relations") | .wall_ns] | first) | add / length / 1e6),
                            linear_algebra: ($r | map(la(.wall_ns)) | add / length / 1e6) } }
         else {} end);
  ($arms | map(select(startswith("dense-")) | ltrimstr("dense-")) | map(tonumber) | sort) as $sizes
  | { schema: "ic_linalg_pipeline.analysis/v1",
      session_id: $all[0].session_id,
      measured_runs: ($all | length), unverified: $unverified,
      arms: [ $arms[] | arm_summary(.) ],
      per_size: [ $sizes[] as $n
        | { N: $n,
            cado_vs_dense_S: cmp("dense-\($n)"; "cado-\($n)"),
            cado_vs_dense_la: lacmp("dense-\($n)"; "cado-\($n)"),
            dense_vs_rho_S: cmp("rho-neg"; "dense-\($n)"),
            cado_vs_rho_S: cmp("rho-neg"; "cado-\($n)") } ],
      aa_control: (if ($arms | index("cado-4096-aa")) then lacmp("cado-4096"; "cado-4096-aa") else null end) }
