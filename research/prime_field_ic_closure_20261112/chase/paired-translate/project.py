"""Project the paired-translate factor onto the recorded mitm2 incumbent counts (derived, [pcc:unverified])."""
import json, math
A = '/home/user/crypto/research/prime_base_oracle_tuning_20261029/analysis/compare-P{b}-rho-neg__inc-w{w}.json'
N = {20: 32, 24: 162, 28: 407, 32: 1024}
f = 0.516   # parent additions per candidate, measured in the toy (paired, 1/16 skipped copies)
for b in (20, 24, 28, 32):
    d0 = json.load(open(A.format(b=b, w='0'))); d1 = json.load(open(A.format(b=b, w='1')))
    r = d0['workloads'][0]['r']; runs = d0['b']['measured']; sq = math.sqrt(r)
    u = {k: v / runs for k, v in d0['b']['unpriced_counts'].items()}
    tot0 = d0['b']['mean_s'] * sq; tot1 = tot0 + sum(u.values())  # w=1: every unpriced count at 1 GAE
    rho0 = d0['a']['mean_s'] * sq; rho1 = rho0 + sum(d0['a']['unpriced_counts'].values()) / d0['a']['measured']
    C = u['lookups_uncharged']; T = N[b] * (N[b] + 1); O = tot0 - C - T
    p0 = tot0 - C * (1 - f)          # w=0: second point free
    p1 = tot1                        # w=1: second point priced 1 GAE, probes/guards per candidate unchanged
    print(f'    check: S_w1 inc={tot1/sq:.3f}, rho={rho1/sq:.3f}')
    print(f'{b} bits: r={r} GAE/run w0={tot0:,.0f}: walk C={C:,.0f} ({C/tot0:.1%}), table N(N+1)={T:,} ({T/tot0:.1%}), other={O:,.0f} ({O/tot0:.1%}); '
          f'w1 total={tot1:,.0f}: walk {C/tot1:.1%}, probes+guards {(u["lookups_uncharged"]+u["target_guard_probes_uncharged"])/tot1:.1%}; '
          f'C*N/r={C*N[b]/r:.3f}')
    print(f'    IC/rho-neg w=0: {tot0/rho0:.2f} -> paired fixed-N {p0/rho0:.2f} (x{p0/tot0:.3f}); w=1: {tot1/rho1:.2f} -> {p1/rho1:.2f} (x1.000)')
print('rebalanced optimum A/N+N^2 with A scaled by f: total x', round(f ** (2/3), 3))
