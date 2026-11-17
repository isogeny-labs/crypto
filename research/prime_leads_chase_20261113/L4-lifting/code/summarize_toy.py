import json, sys, collections, math
D = sys.argv[1]
def load(f):
    try: return [json.loads(l) for l in open(f'{D}/{f}.jsonl')]
    except FileNotFoundError: return []
H, B, P, AA = load('H'), load('B'), load('P'), load('AA')
def agg(rows):
    g = collections.defaultdict(list)
    for r in rows: g[(r['bits'], r['k'])].append(r)
    return g
gH, gB, gP = agg(H), agg(B), agg(P)
out = []
print('bits k | H: n ok nofit/degen  trivial_trials tors_trials NONTRIV_trials useful correct maxcoefbits | B: n frac_trials_with_nontriv mean_nontriv_rel | P: found/n | expected-H-if-tracking-B')
for key in sorted(set(gH) | set(gB)):
    h = gH.get(key, []); b = gB.get(key, []); pp = gP.get(key, [])
    ok = [r for r in h if r.get('status') == 'ok']
    bad = len(h) - len(ok)
    tr = sum(1 for r in ok if r['triv'] > 0); to = sum(1 for r in ok if r['tors'] > 0)
    nt = sum(1 for r in ok if r['nontriv'] > 0); us = sum(1 for r in ok if r['useful'] > 0); co = sum(1 for r in ok if r['log_correct'] > 0)
    cb = max([r.get('coef_bits', 0) for r in ok] or [0])
    bok = [r for r in b if r.get('status') == 'ok']
    bf = sum(1 for r in bok if r['nontriv'] > 0) / max(1, len(bok))
    bm = sum(r['nontriv'] for r in bok) / max(1, len(bok))
    pf = sum(1 for r in pp if r.get('planted_found')); pn = len(pp)
    exp_track = bf * len(ok)
    allconsist = all(r.get('modp_ok', True) and r.get('exact_ok') in (None, True) for r in ok + pp)
    row = dict(bits=key[0], k=key[1], H_n=len(h), H_ok=len(ok), H_bad=bad, H_triv=tr, H_tors=to, H_nontriv=nt, H_useful=us, H_correct=co,
               H_coefbits=cb, B_n=len(bok), B_frac=bf, B_meanrel=bm, P_found=pf, P_n=pn, B_expected_in_H=exp_track, consistent=allconsist)
    out.append(row)
    print(f"{key[0]:>3} {key[1]} | {len(h)} {len(ok)} {bad} {tr} {to} {nt} {us} {co} {cb} | {len(bok)} {bf:.3f} {bm:.2f} | {pf}/{pn} | {exp_track:.1f} {'' if allconsist else 'INCONSISTENT'}")
json.dump(out, open(f'{D}/summary.json', 'w'), indent=1)
if AA:
    for k in (4, 8):
        a = [r for r in AA if r['k'] == k and r.get('status') == 'ok']; h = [r for r in H if r['k'] == k and r['bits'] == 12 and r.get('status') == 'ok']
        print('A/A 12 bits k=%d: seed2 nontriv %d/%d triv %d ; seed1 nontriv %d/%d triv %d' % (k, sum(r['nontriv'] > 0 for r in a), len(a), sum(r['triv'] > 0 for r in a), sum(r['nontriv'] > 0 for r in h), len(h), sum(r['triv'] > 0 for r in h)))
