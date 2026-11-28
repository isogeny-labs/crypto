#!/usr/bin/env python3
"""Score the panel per PROTOCOL.md; write data/summary.json and data/per_instance.csv."""
import json, math, random, statistics as st, sys
from pathlib import Path

D = Path(__file__).resolve().parents[1] / 'data'
import gzip
rows = [json.loads(l) for l in gzip.open(D / 'panel.jsonl.gz', 'rt')]
COPIES = 5
ids = sorted({r['id'] for r in rows}, key=lambda s: int(s[1:]))
arms = sorted({r['arm'] for r in rows})
M = rows[0]['M']


def per(arm, i, aa=False):
    return [r for r in rows if r['arm'] == arm and r['id'] == i and r['aa'] == aa]


inst = []
all_correct = True
for i in ids:
    rec = dict(id=i, planted=per('B', i)[0]['planted'], n_roots=per('B', i)[0]['n_roots'])
    ok = all(r.get('complete') and r['oracle_match'] for r in rows if r['id'] == i)
    rec['all_arms_equal_oracle'] = ok
    all_correct &= ok
    for a in arms:
        ts = [r['wall'] / COPIES for r in per(a, i)]
        rec[a] = st.median(ts)
        rec[a + '_reps'] = len(ts)
        if a != 'B':
            rec[a + '_solve_s'] = st.median(st.median(r['solve_seconds']) for r in per(a, i))
            rec[a + '_field_ops'] = per(a, i)[0]['field_ops']
        else:
            rec['B_nodes'] = per(a, i)[0]['nodes']
            rec['B_lll_steps'] = per(a, i)[0]['lll_steps']
    f4 = [a for a in arms if a != 'B']
    rec['baseline_arm'] = min(f4, key=lambda a: rec[a])
    rec['baseline'] = rec[rec['baseline_arm']]
    rec['ratio'] = rec['baseline'] / rec['B']
    rec['ratio_vs_ideal'] = rec.get('F4-ideal', math.nan) / rec['B']
    inst.append(rec)


def aa_spread(arm):
    out = []
    for i in ids:
        for rep in (1, 2, 3):
            a = [r['wall'] for r in rows if r['arm'] == arm and r['id'] == i and r['rep'] == rep and not r['aa']]
            b = [r['wall'] for r in rows if r['arm'] == arm and r['id'] == i and r['rep'] == rep and r['aa']]
            if a and b:
                out.append(abs(math.log(a[0] / b[0])))
    out.sort()
    return dict(median=st.median(out), p90=out[int(0.9 * (len(out) - 1))], n=len(out))


def gmean(x):
    return math.exp(sum(map(math.log, x)) / len(x))


ratios = [r['ratio'] for r in inst]
rng = random.Random(20261123)
boot = sorted(gmean([rng.choice(ratios) for _ in ratios]) for _ in range(4000))
ideal_ratios = [r['ratio_vs_ideal'] for r in inst if not math.isnan(r['ratio_vs_ideal'])]
aa = {a: aa_spread(a) for a in ('B', 'F4-visit')}
g = gmean(ratios)
conds = dict(
    instances=len(inst), repetitions=min(r['B_reps'] for r in inst),
    all_root_sets_equal=all_correct,
    fastest_baseline_ge_1s=sum(r['baseline'] >= 1.0 for r in inst),
    aa_median_le_10pct=all(v['median'] <= math.log(1.10) for v in aa.values()),
    worst_ratio=min(ratios), worst_ok=min(ratios) >= 1 / 1.10,
)
def gate(k):
    measured = (g >= k and conds['worst_ok'] and conds['all_root_sets_equal']
                and conds['fastest_baseline_ge_1s'] == len(inst) and conds['aa_median_le_10pct']
                and conds['repetitions'] >= 3 and len(inst) == 20)
    return 'measured conditions met, gate not released (isolation/coverage unmet)' if measured else 'FAIL on measured conditions'
summary = dict(
    M=M, p=4294967291, geomean_ratio_vs_fastest_F4=g,
    geomean_ci95_instance_bootstrap=[boot[int(0.025 * len(boot))], boot[int(0.975 * len(boot)) - 1]],
    geomean_ratio_vs_F4_ideal=gmean(ideal_ratios) if ideal_ratios else None,
    min_ratio=min(ratios), max_ratio=max(ratios),
    baseline_arm_counts={a: sum(r['baseline_arm'] == a for r in inst) for a in arms if a != 'B'},
    median_seconds={a: st.median(r[a] for r in inst) for a in arms},
    median_f4_in_process_solve_seconds={a: st.median(r[a + '_solve_s'] for r in inst) for a in arms if a != 'B'},
    aa_abs_log_spread=aa, conditions=conds,
    release_2x=gate(2), target_8x=gate(8),
    standing='[pcc:unverified] L0 shared host')
(D / 'summary.json').write_text(json.dumps(summary, indent=1))
cols = ['id', 'planted', 'n_roots', 'all_arms_equal_oracle'] + arms + ['baseline_arm', 'ratio', 'ratio_vs_ideal', 'B_nodes', 'B_lll_steps'] + \
       [a + '_solve_s' for a in arms if a != 'B'] + [a + '_field_ops' for a in arms if a != 'B']
with open(D / 'per_instance.csv', 'w') as f:
    f.write(','.join(cols) + '\n')
    for r in inst:
        f.write(','.join(f'{r[c]:.6g}' if isinstance(r[c], float) else str(r[c]) for c in cols) + '\n')
print(json.dumps(summary, indent=1))
