#!/usr/bin/env python3
"""L14 decision rules (PROTOCOL.md section 5), applied verbatim to data/k20_t{4,5,6}.json.
Writes data/summary.json and data/selected_digit_sets.json, prints the P-table.
Rules are frozen here before data; a failed prediction stays failed."""
import json, os, sys
import numpy as np

LEAD = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), '..'))
DATA = os.path.join(LEAD, 'data')
K = 20
TS = (4, 5, 6)
# Reference cells computed before data from research/curve25519_torsion_leads_20261103/data/k20.json
# (translation frames, K=2000, seed 20261124) and curve25519_frame_search_20261103/analysis/summary.json.
REF_BITS_LO0 = {4: 12, 5: 42, 6: 162}
REF_TRANS_MEAN = {4: 19.26, 5: 70.90, 6: 270.54}


def load(t):
    return json.load(open(os.path.join(DATA, f'k{K}_t{t}.json')))


def main():
    cells, P = {}, {}
    sel = []
    for t in TS:
        d = load(t)
        tr = np.array([x['covered'] for x in d['part_a']['translation']])
        dg = np.array([x['covered'] for x in d['part_a']['digit']])
        tr_cols = np.array([x['cols'] for x in d['part_a']['translation']])
        ib = int(tr.argmax())
        g0 = d['greedy'][0]
        gcov = np.array([g['covered'] for g in d['greedy']])
        cost_a = d['part_a']['cost']
        cells[t] = dict(
            bits_lo0=d['bits_lo0']['covered'],
            trans_mean=float(tr.mean()), trans_sd=float(tr.std()), trans_max=int(tr.max()),
            trans_best=d['part_a']['translation'][ib],
            digit_mean=float(dg.mean()), digit_sd=float(dg.std()), digit_max=int(dg.max()),
            digit_distinct_x_min=int(min(x['distinct_x'] for x in d['part_a']['digit'])),
            mean_ratio=float(dg.mean() / tr.mean()), max_ratio=float(dg.max() / tr.max()),
            greedy_rep0=dict(covered=g0['covered'], cols=g0['cols'], J=g0['J'], lo=g0['lo'], digits=g0['digits'],
                             cost=g0['cost'], steps=[(s['j'], s['cols'], s['covered'], s['J']) for s in g0['steps']]),
            greedy_all=[dict(rep=g['rep'], covered=g['covered'], cols=g['cols']) for g in d['greedy']],
            greedy_over_trans_max=float(g0['covered'] / tr.max()),
            greedy_over_trans_mean=float(g0['covered'] / tr.mean()),
            greedy_median_over_trans_max=float(np.median(gcov) / tr.max()),
            greedy_spread=float(gcov.max() / gcov.min()),
            cov_per_col_greedy=g0['covered'] / g0['cols'],
            cov_per_col_trans_best=tr[ib] / tr_cols[ib],
            cost_trans_census=cost_a['translation'],
            greedy_legendre_over_trans=float(g0['cost']['legendre'] / cost_a['translation']['legendre']),
            greedy_rP_over_trans=float(g0['cost']['rP'] / cost_a['translation']['rP']),
            greedy_group_ops=float(g0['cost']['group_ops']),
        )
        for g in d['greedy']:
            sel.append(dict(k=K, t=t, rep=g['rep'], seed=g['seed'], lo=g['lo'], digits=g['digits'],
                            cols=g['cols'], covered=g['covered'], J=g['J']))
    c = cells
    P['P1'] = all(c[t]['bits_lo0'] == REF_BITS_LO0[t] and abs(c[t]['trans_mean'] / REF_TRANS_MEAN[t] - 1) <= 0.10 for t in TS)
    P['P2'] = all(0.95 <= c[t]['mean_ratio'] <= 1.05 for t in TS)
    P['P3'] = all(0.75 <= c[t]['max_ratio'] <= 1.33 for t in TS)
    P['P4'] = all(c[t]['greedy_over_trans_max'] >= 1.3 for t in (5, 6))
    P['P5'] = all(1.10 <= c[t]['greedy_over_trans_max'] <= 1.45 for t in (5, 6))
    P['P6'] = c[4]['greedy_over_trans_mean'] >= 3.0 and all(c[t]['greedy_over_trans_mean'] >= 2.0 for t in (5, 6))
    P['P7'] = all(c[t]['greedy_legendre_over_trans'] <= 1.0 and 0.8 <= c[t]['greedy_rP_over_trans'] <= 1.2 for t in TS)
    P['P8'] = all(c[t]['cov_per_col_greedy'] >= 0.9 * c[t]['cov_per_col_trans_best'] for t in (5, 6))
    P['P9'] = all(c[t]['greedy_spread'] <= 1.3 for t in (5, 6))
    lead_status = 'OPEN: digit-set greedy >= 1.3x translation best (P4 held)' if P['P4'] else \
        'CLOSED at B=2000 greedy: digit sets do not beat translation-frame selection by 1.3x (P4 failed)'
    out = dict(k=K, cells={str(t): cells[t] for t in TS}, predictions=P, lead_status=lead_status,
               ref=dict(bits_lo0=REF_BITS_LO0, trans_mean=REF_TRANS_MEAN))
    json.dump(out, open(os.path.join(DATA, 'summary.json'), 'w'), indent=1, default=float)
    json.dump(sel, open(os.path.join(DATA, 'selected_digit_sets.json'), 'w'), indent=1)
    print(f'{"t":>2} {"lo0":>5} {"tr_mean":>8} {"tr_max":>6} {"dg_mean":>8} {"dg_max":>6} {"mean_r":>6} {"max_r":>6} '
          f'{"greedy0":>7} {"cols":>4} {"g/trmax":>7} {"g/trmean":>8} {"spread":>6} {"leg_r":>6} {"rP_r":>6}')
    for t in TS:
        x = cells[t]
        print(f'{t:>2} {x["bits_lo0"]:>5} {x["trans_mean"]:>8.2f} {x["trans_max"]:>6} {x["digit_mean"]:>8.2f} {x["digit_max"]:>6} '
              f'{x["mean_ratio"]:>6.3f} {x["max_ratio"]:>6.3f} {x["greedy_rep0"]["covered"]:>7} {x["greedy_rep0"]["cols"]:>4} '
              f'{x["greedy_over_trans_max"]:>7.3f} {x["greedy_over_trans_mean"]:>8.2f} {x["greedy_spread"]:>6.3f} '
              f'{x["greedy_legendre_over_trans"]:>6.3f} {x["greedy_rP_over_trans"]:>6.3f}')
    for k, v in P.items():
        print(f'{k}: {"held" if v else "FAILED"}')
    print(lead_status)


if __name__ == '__main__':
    main()
