# Independent instance check for one raw report (validator code, no repository code).
#  * slug icv1-f2m<n>-t<t>|tm<t>-...: group order must be 2^n + 1 - t;
#  * Koblitz curves y^2 + xy = x^3 + a x^2 + 1: t_n recomputed from the Frobenius
#    recurrence t_k = t_1 t_{k-1} - 2 t_{k-2}, t_0 = 2, t_1 = -1 (a = 0) or 1 (a = 1);
#  * r prime (trial division), r | group order, cofactor <= 8 (the sweeps' max_cofactor);
#  * every repeat's recovered logarithm lies in [1, r) and equals the planted value the
#    same process's rho runs record for that repeat (known-answer link).
# Usage: jq --arg name koblitz-n13 -f instance_check.jq results/koblitz-n13.json
def isprime: . as $n | if $n < 2 then false
  else ([range(2; ($n | sqrt | floor) + 1)] | all(. as $d | $n % $d != 0)) end;
def koblitz_trace($n; $a):
  (if $a == 0 then -1 else 1 end) as $t1
  | reduce range(2; $n + 1) as $k ([2, $t1]; [.[1], $t1 * .[1] - 2 * .[0]]) | .[1];
. as $rep
| ($rep.instance | capture("f2m(?<n>[0-9]+)-t(?<m>m?)(?<t>[0-9]+)-")) as $c
| ($c.n | tonumber) as $n
| ((if $c.m == "m" then -1 else 1 end) * ($c.t | tonumber)) as $t
| ($rep.rows[0].group_order) as $N
| ($rep.rows[0].r) as $r
| ([ $rep.rows[] | {g: .group_order, r: .r} ] | unique | length == 1) as $one_instance
| {
    name: $name,
    instance: $rep.instance,
    regime: $rep.regime,
    n: $n, trace_from_slug: $t, group_order: $N, r: $r,
    all_rows_same_instance: $one_instance,
    order_equals_2n_plus_1_minus_t: ($N == (pow(2; $n) + 1 - $t)),
    r_prime: ($r | isprime),
    r_divides_order: ($N % $r == 0),
    cofactor: ($N / $r),
    cofactor_le_8: (($N / $r) <= 8),
    koblitz_trace_a0: (if $rep.regime == "koblitz" then koblitz_trace($n; 0) else null end),
    koblitz_trace_a1: (if $rep.regime == "koblitz" then koblitz_trace($n; 1) else null end),
    koblitz_trace_matches: (if $rep.regime == "koblitz" then (koblitz_trace($n; 0) == $t or koblitz_trace($n; 1) == $t) else null end),
    known_answer: [ range(0; $rep.rows | length) as $i | $rep.rows[$i] as $row
      | (($i % 3)) as $k
      | { row: $i, seed: $row.spec.seed, recovered: $row.recovered,
          planted_for_repeat: $rep.rho_reference.per_run[$k].planted,
          in_range: ($row.recovered >= 1 and $row.recovered < $r),
          equals_planted: ($row.recovered == $rep.rho_reference.per_run[$k].planted),
          verified_flag: $row.verified } ],
    known_answer_all_ok: ([ range(0; $rep.rows | length) as $i | $rep.rows[$i] as $row
      | ($row.recovered == $rep.rho_reference.per_run[$i % 3].planted) and $row.verified
        and $row.recovered >= 1 and $row.recovered < $r ] | all),
    rho_planted_cycle_ok: ([ range(0; $rep.rho_reference.per_run | length) as $j
      | $rep.rho_reference.per_run[$j].planted == $rep.rho_reference.per_run[$j % 3].planted ] | all)
  }
