# PROTOCOL.md derivation rule, per family. Input: the six Phase A band JSONs
# (slurped, order b21 b23 b25 k23 k31 k37). G(m,w) = geometric mean over the
# family's curves of mean S_w(arm for m) / mean S_w(plan); m*(w) = argmin_m G,
# ties to the smaller m. Binary: arm d<k> for m = 2^k. Koblitz: the arm with
# max(1, round(m c0)) columns, c0 the plan's column count.
def means: .rows | map({(.arm): .mean_s}) | add;
. as $all
| ($all[0].weights) as $W
| ($all[0:3]) as $bin | ($all[3:6]) as $kob
| { binary: { grid: [0.25, 0.5, 1, 2, 4],
              arms: {"0.25": "dm2", "0.5": "dm1", "1": "plan", "2": "dp1", "4": "dp2"} },
    koblitz: { grid: [0.5, 0.71, 1, 1.41, 2, 2.83, 4], c0: {k23: 3, k31: 2, k37: 10} } } as $cfg
| { schema: "ic_planner_weight_rule_binkob.derivation/v1", weights: $W,
    binary: [ $W | to_entries[] | .key as $k | .value as $w
      | [ $cfg.binary.grid[] as $m
          | { m: $m, G: ([ $bin[] | means as $s | ($s[$cfg.binary.arms[$m|tostring]][$k] / $s["plan"][$k]) | log ] | add / length | exp) } ]
      | { w: $w, grid: ., best: (sort_by(.G, .m) | first) } ],
    koblitz: [ $W | to_entries[] | .key as $k | .value as $w
      | [ $cfg.koblitz.grid[] as $m
          | { m: $m, G: ([ range(0; 3) as $i | $kob[$i] | means as $s
                | ([$cfg.koblitz.c0.k23, $cfg.koblitz.c0.k31, $cfg.koblitz.c0.k37][$i]) as $c0
                | ([1, ($m * $c0 | round)] | max) as $c
                | (if $c == $c0 then "plan" else "c\($c)" end) as $arm
                | ($s[$arm][$k] / $s["plan"][$k]) | log ] | add / length | exp) } ]
      | { w: $w, grid: ., best: (sort_by(.G, .m) | first) } ] }
| .rule = { binary: [ .binary[] | {w, m_star: .best.m, G: .best.G} ],
            koblitz: [ .koblitz[] | {w, m_star: .best.m, G: .best.G} ] }
