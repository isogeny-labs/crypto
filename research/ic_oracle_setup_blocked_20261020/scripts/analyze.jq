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
def phase($n; f): [ .phases[] | select(.name == $n) | f ] | first;
def setup_ns: phase("oracle_setup"; .wall_ns);
def run_ns: .time.solve_wall_ns;
def key: "\(.workload.workload_id)/\(.round)";
def stats: if length == 0 then null else sort as $s | { n: length, min: $s[0], median: $s[(length/2|floor)], max: $s[-1] } end;
def ratio_sum($a; $b; f): if ($a | length) == 0 then null else ([ $b[] | f ] | add) / ([ $a[] | f ] | add) end;
# What must agree exactly between two oracles of one size (PROTOCOL measure 1):
# the answer, the counted cost, every phase's counted operations and native
# counters except wall-clock fields, and the run's counters.
def counted: { recovered: .outcome.recovered, status: .outcome.status,
               total_gae: .cost.total_gae, counters: .counters,
               phases: [ .phases[] | { name, adds, doubles, scalar_mults, gae, native } ] };

[ .[] | select(.warmup == false) ] as $all
| [ .[] ] as $every
| ($all | map(select(.outcome.status != "verified")) | length) as $unverified
| ($all | map({ (key + "|" + .arm): . }) | add) as $by
| ($every | map({ ("\(.workload.workload_id)/\(.round)/\(.warmup)|" + .arm): . }) | add) as $by_every
| ($all | map(.arm) | unique) as $arms
| ($all | map(key) | unique) as $keys
| ($every | map("\(.workload.workload_id)/\(.round)/\(.warmup)") | unique) as $keys_every
| def pairs($a; $b): [ $keys[] as $k | select($by[$k+"|"+$a] and $by[$k+"|"+$b]) | [ $by[$k+"|"+$a], $by[$k+"|"+$b] ] ];
  def equal($a; $b):
    [ $keys_every[] as $k | select($by_every[$k+"|"+$a] and $by_every[$k+"|"+$b])
      | ($by_every[$k+"|"+$a] | counted) == ($by_every[$k+"|"+$b] | counted) ]
    | { executions: length, identical: (map(select(.)) | length) };
  def cmp($a; $b):
    pairs($a; $b) as $p
    | ($p | map(.[0])) as $A | ($p | map(.[1])) as $B
    | { a: $a, b: $b, pairs: ($p | length),
        S_ratio: { w0: ratio_sum($A; $B; s_w(0)), w0_1: ratio_sum($A; $B; s_w(0.1)), w1: ratio_sum($A; $B; s_w(1)) } };
  def wallcmp($a; $b):
    pairs($a; $b) as $p
    | ($p | map(.[0])) as $A | ($p | map(.[1])) as $B
    | { a: $a, b: $b, pairs: ($p | length),
        setup_wall_ratio_sum: ratio_sum($A; $B; setup_ns),
        setup_wall_pair_ratio: ($p | map((.[1] | setup_ns) / (.[0] | setup_ns)) | stats),
        run_wall_ratio_sum: ratio_sum($A; $B; run_ns),
        run_wall_pair_ratio: ($p | map((.[1] | run_ns) / (.[0] | run_ns)) | stats) };
  def arm_summary($arm):
    [ $all[] | select(.arm == $arm) ] as $r
    | { arm: $arm, runs: ($r | length),
        verified: ($r | map(select(.outcome.status == "verified")) | length),
        levels: ($r | map(.isolation.level) | group_by(.) | map({ (.[0]): length }) | add),
        S_w0_mean: ($r | map(s_w(0)) | add / length),
        S_w0_1_mean: ($r | map(s_w(0.1)) | add / length),
        S_w1_mean: ($r | map(s_w(1)) | add / length),
        gae_mean: ($r | map(.cost.total_gae) | add / length),
        uncharged_mean: ($r | map(uncharged) | add / length),
        run_wall_ms_mean: ($r | map(run_ns) | add / length / 1e6),
        max_rss_kib: ($r | map(.time.max_rss_kib) | stats) }
      + (if $r[0].method.id == "ic.pipeline" then
          { oracle: $r[0].method.params.oracle, factor_base: $r[0].method.params.factor_base,
            setup_gae_mean: ($r | map(phase("oracle_setup"; .gae)) | add / length),
            relations_gae_mean: ($r | map(phase("relations"; .gae)) | add / length),
            relations: ($r | map(phase("relations"; .native.relations)) | stats),
            wall_ms_mean: { oracle_setup: ($r | map(setup_ns) | add / length / 1e6),
                            relations: ($r | map(phase("relations"; .wall_ns)) | add / length / 1e6),
                            linear_algebra: ($r | map(phase("linear_algebra"; .wall_ns)) | add / length / 1e6) } }
         else {} end);
  ($arms | map(select(startswith("blocked-") and (endswith("-aa") | not)) | ltrimstr("blocked-")) | map(tonumber) | sort) as $sizes
  | { schema: "ic_oracle_setup_blocked.analysis/v1",
      session_id: $all[0].session_id,
      measured_runs: ($all | length), all_records: ($every | length), unverified: $unverified,
      arms: [ $arms[] | arm_summary(.) ],
      per_size: [ $sizes[] as $n
        | { N: $n,
            equality: { batch_vs_scalar: equal("scalar-\($n)"; "batch-\($n)"),
                        blocked_vs_batch: equal("batch-\($n)"; "blocked-\($n)") },
            blocked_vs_batch_S: cmp("batch-\($n)"; "blocked-\($n)"),
            blocked_vs_scalar_S: cmp("scalar-\($n)"; "blocked-\($n)"),
            blocked_vs_batch_wall: wallcmp("batch-\($n)"; "blocked-\($n)"),
            blocked_vs_scalar_wall: wallcmp("scalar-\($n)"; "blocked-\($n)"),
            batch_vs_scalar_wall: wallcmp("scalar-\($n)"; "batch-\($n)"),
            blocked_vs_rho_S: cmp("rho-neg"; "blocked-\($n)") } ],
      aa_control: (if ($arms | index("blocked-4096-aa")) then
                     { equality: equal("blocked-4096"; "blocked-4096-aa"),
                       wall: wallcmp("blocked-4096"; "blocked-4096-aa") } else null end) }
