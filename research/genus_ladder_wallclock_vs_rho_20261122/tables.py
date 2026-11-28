#!/usr/bin/env python3
import json
A = json.load(open('analysis.json')); PR = ['65537', '131101', '262147']
IC, RH, NS = 'C_double_lp+pre+sge(8,3)', 'R_rho_neg', 'C_double_lp+pre'
PRED = {'65537': 0.176, '131101': 0.228, '262147': 0.198}
o = []; w = o.append
w('## T1 wall ratio IC(2LP+pre+SGE(8,3)) / negation rho, paired, 6 curves x 2 seeds per prime, 95% hierarchical bootstrap `[pcc:unverified]`\n')
w('| p | mean wall IC (s) | mean wall rho (s) | ratio, new binary, 1 thread | 95% interval | R8 count re-pricing | ratio without SGE | old binary (R2, threads 4) ratio | old interval |')
w('|--:|--:|--:|--:|:--|--:|--:|--:|:--|')
for p in PR:
    P = A['primes'][p]; n = P['new_ratio']; d = P['old_ratio']
    w(f"| {p} | {P['mean_wall_s_new_'+IC]:.2f} | {P['mean_wall_s_new_'+RH]:.2f} | {n[0][0]:.3f} | [{n[1]:.3f}, {n[2]:.3f}] | {PRED[p]:.3f} | {P['new_noSGE_ratio'][0]:.3f} | {d[0][0]:.3f} | [{d[1]:.3f}, {d[2]:.3f}] |")
w('\n## T2 mean wall per arm (s): new binary (this run, 1 thread, host otherwise idle) vs old binary (R2, 4 runs in parallel)\n')
w('| p | IC+SGE new | IC+SGE old | IC no SGE new | IC no SGE old | rho new | rho old |')
w('|--:|--:|--:|--:|--:|--:|--:|')
for p in PR:
    P = A['primes'][p]
    w(f"| {p} | {P['mean_wall_s_new_'+IC]:.2f} | {P['mean_wall_s_old_'+IC]:.2f} | {P['mean_wall_s_new_'+NS]:.2f} | {P['mean_wall_s_old_'+NS]:.2f} | {P['mean_wall_s_new_'+RH]:.2f} | {P['mean_wall_s_old_'+RH]:.2f} |")
w('\n## T3 ns per counted operation (new binary)\n')
w('Counted op = Cantor field multiplication + 10 x inversion (rho: all its counted work; IC relation phase: that plus `rel_mul`). IC relation-phase wall is not recorded separately: it is estimated as the arm total minus `la_mul` x R8 ns per `la_mul` (16.6/20.3/19.0 ns), so the IC relation-phase figure is an estimate.\n')
w('| p | rho ns / counted op | rho ns / walk step | IC relation-phase ns / counted op (est.) | IC total ns / FPM_mid op (kappa 9) | est. LA share of IC wall | IC wall / `la_mul` (total, ns) |')
w('|--:|--:|--:|--:|--:|--:|--:|')
for p in PR:
    P = A['primes'][p]
    w(f"| {p} | {P['rho_ns_per_fpm']:.2f} | {P['rho_ns_per_step']:.0f} | {P['ic_rel_ns_per_op_est']:.2f} | {P['ic_ns_per_fpm_total']:.2f} | {P['ic_la_wall_share_est']:.2f} | {P['ic_la_ns_per_la_mul_total']:.1f} |")
w('\n## T4 per-curve paired ratio (mean over 2 seeds)\n')
w('| p | curve seed | IC s | rho s | ratio |'); w('|--:|--:|--:|--:|--:|')
for p in PR:
    for c, v in A['primes'][p]['per_instance'].items():
        w(f"| {p} | {c} | {v['ic_s']:.2f} | {v['rho_s']:.2f} | {v['ratio']:.3f} |")
e = A['exponents_wall_vs_p']
w('\n## T5 exponent of mean wall in p (3 primes, descriptive, no interval)\n')
w('| arm | new binary | old binary (R2) |'); w('|--|--:|--:|')
w(f"| IC + SGE(8,3) | {e['new_'+IC]:.2f} | {e['old_'+IC]:.2f} |"); w(f"| negation rho | {e['new_'+RH]:.2f} | {e['old_'+RH]:.2f} |")
w('\n## T6 V1 identity check against R2 (count fields of all 108 runs)\n')
w('| p | runs | all certified | non-wall field differences | instance lines equal |'); w('|--:|--:|:-:|--:|:-:|')
for p in PR:
    v = A['v1'][p]; w(f"| {p} | {v['runs']} | {v['all_certified']} | {v['n_diffs']} | {v['instances_equal']} |")
open('tables.md', 'w').write('\n'.join(o) + '\n')
# svg
SURF, INK, INK2, GRID = '#fcfcfb', '#0b0b0b', '#52514e', '#e4e3df'
W, H = 760, 420; L, B, T = 70, 70, 50; ymax = 1.0
def Y(v): return H - B - (H - B - T) * v / ymax
s = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="12"><rect width="{W}" height="{H}" fill="{SURF}"/>']
s.append(f'<text x="{L}" y="26" font-size="15" fill="{INK}">Wall-time ratio: IC (2LP + prefilter + SGE(8,3)) / negation rho, genus 3</text>')
for v in (0, .2, .4, .6, .8, 1.0):
    s.append(f'<line x1="{L}" x2="{W-20}" y1="{Y(v)}" y2="{Y(v)}" stroke="{GRID}"/><text x="{L-8}" y="{Y(v)+4}" text-anchor="end" fill="{INK2}">{v:.1f}</text>')
cols = [('new', '#2a78d6', 'new binary, 1 thread'), ('old', '#eb6834', 'old binary (R2, threads 4)')]
gw = (W - L - 20) / 3
for i, p in enumerate(PR):
    P = A['primes'][p]; x0 = L + i * gw + gw * .15
    for j, (k, c, _) in enumerate(cols):
        r = P[k + '_ratio']; x = x0 + j * gw * .32; bw = gw * .26
        s.append(f'<rect x="{x:.1f}" y="{Y(r[0][0]):.1f}" width="{bw:.1f}" height="{Y(0)-Y(r[0][0]):.1f}" fill="{c}"/>')
        xm = x + bw / 2
        s.append(f'<line x1="{xm:.1f}" x2="{xm:.1f}" y1="{Y(r[1]):.1f}" y2="{Y(r[2]):.1f}" stroke="{INK}" stroke-width="1.5"/>')
        s.append(f'<text x="{xm:.1f}" y="{Y(r[2])-5:.1f}" text-anchor="middle" fill="{INK}">{r[0][0]:.2f}</text>')
    xp = x0 + 2 * gw * .32 + gw * .05
    s.append(f'<line x1="{x0-4:.1f}" x2="{x0+gw*.62:.1f}" y1="{Y(PRED[p]):.1f}" y2="{Y(PRED[p]):.1f}" stroke="#1baf7a" stroke-width="2" stroke-dasharray="5 3"/>')
    s.append(f'<text x="{x0+gw*.3:.1f}" y="{H-B+20}" text-anchor="middle" fill="{INK}">p = {p}</text>')
s.append(f'<line x1="{L}" x2="{W-20}" y1="{Y(0)}" y2="{Y(0)}" stroke="{INK2}"/>')
lx = L
for c, t in [('#2a78d6', 'new binary, 1 thread'), ('#eb6834', 'old binary (R2 rows, 4 runs parallel)')]:
    s.append(f'<rect x="{lx}" y="{H-24}" width="12" height="12" fill="{c}"/><text x="{lx+18}" y="{H-14}" fill="{INK2}">{t}</text>'); lx += 230
s.append(f'<line x1="{lx}" x2="{lx+22}" y1="{H-18}" y2="{H-18}" stroke="#1baf7a" stroke-width="2" stroke-dasharray="5 3"/><text x="{lx+28}" y="{H-14}" fill="{INK2}">R8 count re-pricing</text>')
s.append(f'<text x="{L}" y="{T-8}" fill="{INK2}">bars: ratio of means; whiskers: 95% hierarchical bootstrap (6 curves x 2 seeds). [pcc:unverified]</text>')
s.append('</svg>')
open('fig_ratio.svg', 'w').write('\n'.join(s))
