#!/usr/bin/env python3
"""Cross-check counted_gb.solve against msolve 0.6.5 on descended S4 systems (F_p solution sets).
usage: test_vs_msolve.py p seed ncurves nsys"""
import os, random, sys, time, json
HERE = os.path.dirname(os.path.abspath(__file__)); sys.path.insert(0, HERE)
os.environ.setdefault('SCR', '/tmp/claude-0/-home-user/c4b817f7-0ad0-518c-befe-15465b1e9e2a/scratchpad/fp3_full')
import ic_fp3_orig as T, ic_fp2 as F, counted_gb as G
T.SCR = os.environ['SCR']
p = int(sys.argv[1]); seed = int(sys.argv[2]); nc = int(sys.argv[3]); ns = int(sys.argv[4])
T.setp(p); T.find_field(); rng = random.Random(seed)
agree = dis = 0; st = {}; mats = None; per = []
for c in range(nc):
    a, b, r = T.find_curve(rng); S = T.Sys(a, b)
    for k in range(ns):
        P = T.rand_point(a, b, rng)
        polys = S.build(P[0])
        pd = [{(i, j, kk): c_ for i, j, kk, c_ in Pl} for Pl in polys]
        G.PH.clear(); F.C[0] = F.C[1] = 0
        sols, D, status, stt = G.solve(pd, p, rng, lambda f: F.proots(F.ptrim(f), rng))
        tot = sum(v[0] for v in G.PH.values()) + F.C[0]
        per.append(dict(D=D, status=status, M=tot, rounds=stt.get('rounds'), nontriv=stt.get('nontriv')))
        st[status] = st.get(status, 0) + 1
        if mats is None: mats = stt['mat']
        try:
            ms, _, _, mst = T.solve_sys(polys, rng, f'chk{seed}')
        except (TypeError, IndexError):   # msolve output the original parser cannot read
            ms, mst = [], 'parse_error'
        chk = lambda e: all(sum(c_*pow(e[0],i,p)*pow(e[1],j,p)*pow(e[2],kk,p) for i,j,kk,c_ in Pl) % p == 0 for Pl in polys)
        bad = [e for e in sols if not chk(e)]
        st['mine_unverified_sols'] = st.get('mine_unverified_sols', 0) + len(bad)
        a1 = sorted(e for e in sols if chk(e)); a2 = sorted(ms)
        if mst != 'ok' or status != 'ok':
            st['pair:' + status + '/' + mst] = st.get('pair:' + status + '/' + mst, 0) + 1
            continue
        if a1 == a2: agree += 1
        else:
            dis += 1
            only_mine = [e for e in a1 if e not in a2]; only_ms = [e for e in a2 if e not in a1]
            st['only_mine'] = st.get('only_mine', 0) + len(only_mine); st['only_msolve'] = st.get('only_msolve', 0) + len(only_ms)
print(json.dumps(dict(p=p, seed=seed, agree=agree, disagree=dis, status=st, example_f4_matrices=mats,
                      mean_M=sum(x['M'] for x in per)/len(per), D_values=sorted(set(x['D'] for x in per)),
                      nontriv=sorted(set(x['nontriv'] for x in per if x['nontriv'] is not None)))))
