"""R5 analysis: WDSat arm against the lab arm (same session) and the published msolve arm (E1 records).

Pricing rule of PILOT_20261030 section 9.3 and E1: per target, whole = ecbench child instructions retired
(shim `*.time`, one per child run, in record order) + every external solver process's instructions
(here `wdsat_ir`) - 3.51e7 (child start-up).  Solver-only lower bound for WDSat = wdsat_ir - calls * start-up
of one WDSat process (measured on a contradictory one-line instance: 1.35e7 (l=4), 1.39e7 (l=5)).

usage: r5_analyze.py <out.json> <e1_res.json> <cell>@<session_dir> ...      (tm dir = <session_dir>_tm)
cell is one of "K0 n=9" "K1 n=9" "K1 n=11" "K0 n=13" written with underscores: K0_n=9 ...
"""
import json, glob, sys, statistics as st

ECB_START = 3.51e7
WD_START = {4: 1.348e7, 5: 1.385e7, 6: 1.391e7}

def ir(p):
    for l in open(p):
        if 'instructions retired' in l:
            return int(l.split()[0])

def load(sess):
    recs = [json.loads(l) for l in open(sess + '/records.jsonl')]
    tms = sorted(glob.glob(sess + '_tm/*.time'))
    assert len(tms) == len(recs), (sess, len(tms), len(recs))
    out = []
    for i, r in enumerate(recs):
        ex = (r.get('solver') or {}).get('extra') or {}
        l = int(r['method']['params']['factor_base'].split('=')[1])
        wi, wc = ex.get('wdsat_ir', 0), ex.get('wdsat_calls', 0)
        out.append(dict(
            arm=r['arm'], l=l, t=r['workload']['target_index'], ok=r['outcome']['status'] == 'verified',
            whole=ir(tms[i]) + wi - ECB_START, solver_ir=wi, solver_only=wi - wc * WD_START.get(l, 0),
            calls=r['solver']['calls'], wd_calls=wc, inexact=ex.get('wdsat_inexact', 0),
            unsat=ex.get('wdsat_unsat', 0), models=ex.get('wdsat_models', 0),
            S=r['cost']['s'], r=r['workload']['curve']['r']))
    return out

def med(v):
    return st.median(v)

def main():
    outp, e1p = sys.argv[1], sys.argv[2]
    e1 = json.load(open(e1p))
    res = {}
    for spec in sys.argv[3:]:
        cell, sess = spec.split('@', 1)
        cell = cell.replace('_', ' ')
        rows = load(sess)
        by = {}
        for x in rows:
            by.setdefault(x['arm'], {})[x['t']] = x
        c = dict(r=rows[0]['r'], arms={}, pairs={}, e1={})
        for a, d in by.items():
            v = list(d.values())
            ok = [x for x in v if x['ok']]
            c['arms'][a] = dict(
                targets=len(v), verified=len(ok), whole_med=med([x['whole'] for x in ok]),
                solver_only_med=med([x['solver_only'] for x in ok]) if a.startswith('wdsat') else None,
                calls_med=med([x['calls'] for x in ok]), unsat_frac=(sum(x['unsat'] for x in ok) / max(1, sum(x['wd_calls'] for x in ok))) if a.startswith('wdsat') else None,
                inexact=sum(x['inexact'] for x in v))
        for nm, l in [(nm, l) for nm in ('wdsat', 'wdsatf') for l in (4, 5)]:
            lab, wd = by.get(f'inherited-f4-l{l}'), by.get(f'{nm}-l{l}')
            if not lab or not wd:
                continue
            ts = sorted(set(lab) & set(wd))
            rw = [wd[t]['whole'] / lab[t]['whole'] for t in ts if wd[t]['ok'] and lab[t]['ok']]
            rl = [wd[t]['solver_only'] / lab[t]['whole'] for t in ts if wd[t]['ok'] and lab[t]['ok']]
            c['pairs'][f'{nm}-l{l}|inherited-f4-l{l}'] = dict(
                n=len(rw), whole_med=med(rw), whole_range=[min(rw), max(rw)], solver_only_med=med(rl), solver_only_range=[min(rl), max(rl)],
                same_calls=all(lab[t]['calls'] == wd[t]['calls'] for t in ts), same_S=all(abs(lab[t]['S'] - wd[t]['S']) < 1e-9 for t in ts))
        k = cell
        if k in e1:
            E = e1[k]
            c['e1'] = dict(lab_med=E['lab_med'], pub_med=E['pub_med'], pub_low=E['pub_low'], verified=E['verified'])
            lab_aa = {}
            for l in (4, 5):
                a = f'inherited-f4-l{l}'
                if a in by and a in E['lab_med']:
                    lab_aa[a] = by[a] and med([x['whole'] for x in by[a].values()]) / E['lab_med'][a]
            c['lab_aa_vs_e1'] = lab_aa
            for nm in ('wdsat', 'wdsatf'):
                wdl4 = c['arms'].get(f'{nm}-l4')
                if not wdl4:
                    continue
                pm = E['pub_med'].get('msolve-x-l4-s4')
                pl = E['pub_low'].get('msolve-x-l4-s4')
                lbest = min(E['lab_med'].values())
                c.setdefault('derived', {})[nm] = dict(
                    msolve_whole_over_wdsat_whole=pm / wdl4['whole_med'] if pm else None,
                    msolve_solver_only_over_wdsat_solver_only=pl / wdl4['solver_only_med'] if pl else None,
                    wdsat_l4_whole_over_lab_best_e1=wdl4['whole_med'] / lbest, lab_best_e1=lbest,
                    wdsat_l4_solver_only_over_lab_best_e1=wdl4['solver_only_med'] / lbest,
                    wdsat_l4_whole_over_lab_l4_e1=wdl4['whole_med'] / E['lab_med']['inherited-f4-l4'])
        res[cell] = c
    json.dump(res, open(outp, 'w'), indent=1)
    for cell, c in res.items():
        print(cell, 'r=%d' % c['r'])
        for a, v in c['arms'].items():
            print('  %-18s verified %d/%d whole %.3g%s calls %.0f%s inexact %d' % (a, v['verified'], v['targets'], v['whole_med'],
                  (' solver-only %.3g' % v['solver_only_med']) if v['solver_only_med'] else '', v['calls_med'],
                  (' unsat %.2f' % v['unsat_frac']) if v['unsat_frac'] is not None else '', v['inexact']))
        for k, p in c['pairs'].items():
            print('  %-30s whole x%.2f (%.2f..%.2f)  solver-only x%.2f  same_calls %s same_S %s' % (k, p['whole_med'], *p['whole_range'], p['solver_only_med'], p['same_calls'], p['same_S']))
        if c.get('lab_aa_vs_e1'):
            print('  lab A/A vs E1 (ratio of medians):', {k: round(v, 4) for k, v in c['lab_aa_vs_e1'].items()})
        for nm, d in c.get('derived', {}).items():
            print('  derived', nm, {k: (round(v, 3) if v and v < 1e6 else v) for k, v in d.items()})

main()
