# Q3: frozen arm $a against its _v2 arm $b, matched on (workload, round, warm-up).
# jq -s --arg a FROZEN --arg b V2 --arg mode bsgs|kangaroo|large_prime -f pair_check.jq records.jsonl
def close(x; y): ((x - y) | fabs) <= (1e-9 * ([1, (x | fabs)] | max));
def ph($r): ($r.phases | map({key: .name, value: .}) | from_entries);
def bits: if . == 0 then [] else ((. / 2 | floor) | bits) + [. % 2] end;
def key: "\(.workload.workload_id)/\(.round)/\(.warmup)";
def f($p; $n): if $p == null then 0 else $p[$n] // 0 end;
def same($p; $q): ($p == null and $q == null) or
  ($p != null and $q != null and ($p.adds == $q.adds) and ($p.doubles == $q.doubles)
   and ($p.scalar_mults == $q.scalar_mults) and close($p.gae; $q.gae) and (($p.native // {}) == ($q.native // {})));
def sumph($p; $q): if $p == null then $q elif $q == null then $p else
  {adds: ($p.adds + $q.adds), doubles: ($p.doubles + $q.doubles), scalar_mults: ($p.scalar_mults + $q.scalar_mults), gae: ($p.gae + $q.gae), native: (($p.native // {}) + ($q.native // {}))} end;
(map(select(.arm == $a)) | map({key: key, value: .}) | from_entries) as $A
| (map(select(.arm == $b)) | map({key: key, value: .}) | from_entries) as $B
| [ $A | to_entries[] | select($B[.key] != null) | .key as $k | .value as $x | $B[$k] as $y
    | ph($x) as $px | ph($y) as $py
    | ($px | keys) as $kx | ($py | keys) as $ky
    | (if $mode == "kangaroo" then
         # frozen setup = v2 setup + v2 restarts; every other frozen phase equals v2's
         (same($px.setup; sumph($py.setup; $py.restarts)) and
          ([$kx[] | select(. != "setup") | same($px[.]; $py[.])] | all) and
          ([$ky[] | select(. != "setup" and . != "restarts" and . != "verification") | ($px[.] != null)] | all))
       else
         ([$kx[] | select(. != "verification") | same($px[.]; $py[.])] | all) and
         ([$ky[] | select(. != "verification") | ($px[.] != null)] | all)
       end) as $phases_ok
    | ($py.verification.gae // 0) as $vg
    | ($px.verification.gae // 0) as $fg
    | ($y.cost.total_gae - $x.cost.total_gae) as $delta
    | (($x.workload.curve.r | log) / (2 | log)) as $l2r
    | { key: $k, status_ok: ($x.outcome.status == "verified" and $y.outcome.status == "verified"),
        recovered_ok: ($x.outcome.recovered == $y.outcome.recovered),
        seed_ok: ($x.algorithm_seed == $y.algorithm_seed),
        phases_ok: $phases_ok,
        delta: $delta, v2_verification_gae: $vg, frozen_verification_gae: $fg,
        delta_ok: close($delta; $vg - $fg),
        verification_is_one_scalar_mult: (($py.verification.scalar_mults // 0) == 1),
        verification_gae_ok: (if $mode == "large_prime" then null else close($vg; 1.5 * $l2r) end),
        verification_exact_ok: (if $mode == "large_prime" then null else (($y.workload.planted | bits) as $b | (($py.verification.doubles // -1) == ($b | length) - 1) and (($py.verification.adds // -1) == ($b | add) - 1)) end),
        frozen_total: $x.cost.total_gae, v2_total: $y.cost.total_gae, l2r: $l2r } ] as $rows
| { pair: "\($a) -> \($b)", mode: $mode,
    frozen_records: ($A | length), v2_records: ($B | length), matched: ($rows | length),
    unmatched_frozen: (($A | length) - ($rows | length)), unmatched_v2: (($B | length) - ($rows | length)),
    status_ok: ($rows | map(select(.status_ok)) | length),
    recovered_ok: ($rows | map(select(.recovered_ok)) | length),
    seed_ok: ($rows | map(select(.seed_ok)) | length),
    phases_ok: ($rows | map(select(.phases_ok)) | length),
    delta_ok: ($rows | map(select(.delta_ok)) | length),
    one_scalar_mult: ($rows | map(select(.verification_is_one_scalar_mult)) | length),
    verification_gae_ok: ($rows | map(select(.verification_gae_ok == true)) | length),
    verification_exact_ok: ($rows | map(select(.verification_exact_ok == true)) | length),
    mean_verification_over_1p5_log2r: ($rows | map(.v2_verification_gae / (1.5 * .l2r)) | add / length),
    delta_min: ($rows | map(.delta) | min), delta_max: ($rows | map(.delta) | max),
    mean_frozen_total: ($rows | map(.frozen_total) | add / length), mean_v2_total: ($rows | map(.v2_total) | add / length),
    rows: ($rows | map({key, delta, v2_verification_gae, frozen_verification_gae})),
    failures: ($rows | map(select((.status_ok and .recovered_ok and .seed_ok and .phases_ok and .delta_ok) | not)) | .[0:5]) }
