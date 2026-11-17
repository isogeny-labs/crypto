"""Print the key numbers (for the hand-off) straight from out/summary.json.  [pcc:unverified]"""
import os, json, math
OUT = os.path.join(os.path.dirname(__file__), '..', 'out')
S = json.load(open(os.path.join(OUT, 'summary.json')))
X = sorted([s for s in S if s['arm'] == 'X'], key=lambda s: (s['bits'], s['r']))
tot = sum(s['n'] for s in X)
rel = sum(s['relations'] for s in X)
triv = sum(s['rel_trivial'] for s in X)
nt = sum(s['rel_nontrivial'] for s in X)
succ = sum(s['successes'] for s in X)
succ_nt = sum(s['successes_nontrivial'] for s in X)
coll = sum(s['collision'] for s in X)
collp = sum(s['collision_pred'] for s in X)
vf = sum(s['verify_failures'] for s in X)
print(f'honest arm: trials {tot}, relations {rel} (trivial {triv}, non-trivial {nt}), successes {succ} (non-trivial {succ_nt}), collisions {coll} (pred {collp:.1f}), false sieve candidates rejected {vf}')
pats = {}
for s in X:
    for k, v in s['nontrivial_patterns'].items():
        pats[k] = pats.get(k, 0) + v
print('non-trivial relation patterns (all cells):', pats)
print('per bits: non-trivial relations / trials')
for b in sorted(set(s['bits'] for s in X)):
    cells = [s for s in X if s['bits'] == b]
    n = sum(s['n'] for s in cells)
    k = sum(s['rel_nontrivial'] for s in cells)
    q = cells[0]['q_mean']
    print(f'  bits {b}: q~{q:.0f}  {k}/{n} = {k/n:.2e}')
xc = [s['xcert_log2_min'] for s in X if s['xcert_log2_min'] is not None]
xm = [s['xcert_log2_median'] for s in X if s['xcert_log2_median'] is not None]
print(f'certificate: min over cells of min log2 X_cert = {min(xc):.1f}; median over cells of median = {sorted(xm)[len(xm)//2]:.1f}')
rhos = [(s['bits'], s['r'], s['rho_G_median'], s['rho_G_max'], s['lang_lamG_median'], s['lang_lamG_min'], s['H_over_logDW_median'], s['h_n']) for s in X if s.get('h_n')]
print('rho_G median range', min(r[2] for r in rhos), max(r[2] for r in rhos), 'max', max(r[3] for r in rhos))
print('lambda_G/log|Delta_W| median range', min(r[4] for r in rhos), max(r[4] for r in rhos), 'min', min(r[5] for r in rhos))
print('H/log|Delta_W| median range', min(r[6] for r in rhos), max(r[6] for r in rhos), 'heights trials', sum(r[7] for r in rhos))
for arm in ('PC', 'PS', 'PD', 'POS'):
    for s in sorted([s for s in S if s['arm'] == arm], key=lambda s: (s['r'], s['bits'])):
        ci = s.get('planted_ci')
        extra = ''
        if arm in ('PC', 'POS'):
            extra = f" collinear-agree {s.get('collinear_agree')}/{s.get('planted_n')} detected {s.get('planted_detected')}"
        print(f"{arm} r={s['r']} bits={s['bits']} p~{s['p_mean']:.0f}: realised {s['planted_realized']}/{s['planted_n']} = {s['planted_rate']:.2e} CI [{ci[0]:.1e},{ci[1]:.1e}] rate*p={s['planted_rate']*s['p_mean']:.2f}{extra}")
hc = sum(s.get('h_rank_certified_norel', 0) for s in X)
hn = sum(s.get('h_norel', 0) for s in X)
ht = sum(s.get('h_trials', 0) for s in X)
print(f'height subsample: {ht} trials; {hn} with no relation found; rank r certified (lambda_min(G) > 1.5 r eps) on {hc} of those')
for s in X:
    if s.get('h_trials'):
        print(f"  bits {s['bits']} r {s['r']}: certified {s['h_rank_certified_norel']}/{s['h_norel']} (no-relation trials), heights trials {s['h_trials']}")
