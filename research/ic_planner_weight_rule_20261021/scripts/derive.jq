# PROTOCOL.md derivation rule. Input: the three Phase A band JSONs (slurped).
# G(m,w) = geometric mean over curves of mean S_w(m) / mean S_w(m100);
# m*(w) = argmin_m G (ties to the smaller m).
def mult: {"m050":0.5,"m071":0.71,"m100":1,"m141":1.41,"m200":2,"m283":2.83,"m400":4};
. as $bands
| ($bands[0].weights) as $W
| [ $W | to_entries[] | .key as $k | .value as $w
    | [ mult | to_entries[] | .key as $arm | .value as $m
        | { arm: $arm, m: $m,
            per_curve: [ $bands[] | (.rows | map({(.arm): .mean_s[$k]}) | add) as $s | ($s[$arm] / $s["m100"]) ],
          }
        | .G = ((.per_curve | map(log) | add / length) | exp) ]
    | { w: $w, grid: ., best: (sort_by(.G, .m) | first | {arm, m, G}) } ]
| { schema: "ic_planner_weight_rule.derivation/v1", weights: $W, rule: map({w, m_star: .best.m, arm: .best.arm, G: .best.G}), detail: . }
