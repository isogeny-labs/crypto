# Lays out recorded values (results/summary_all.json) into REPORT.typ. No computation.
import json
rows=[json.loads(l) for l in open('results/summary_all.json')]
B=sorted([r for r in rows if r.get('arm')=='B0' and 'q' in r],key=lambda r:r['q'])
A=sorted([r for r in rows if r.get('arm')=='A0r' and 'q' in r],key=lambda r:r['q'])
fits={r['fit']:r for r in rows if 'fit' in r}
P=[json.loads(l) for l in open('results/summary_primary.json')]
pf=[r for r in P if r.get('fit')=='B0'][0]
def f3(x): return '%.3g'%x
t=[]
t.append('#set page(margin: 1.6cm)\n#set text(size: 9.5pt)\n#set heading(numbering: "1.")\n')
t.append('= N4: relation-collection index calculus on the GHS plane quartic vs a matched rho on E0(K)\n')
t.append('2026-10-19. Branch `claude/quartic-ic-n4-20261019`, protocol `PROTOCOL.md` (commit 9a63c9161), records `results/`. Descriptive run report; interpretation in `README.md`. Measured values are in F_q-multiplication equivalents (M-eq); every IC run verified kG = Q on E0.\n')
t.append('== Result in brief\n')
t.append('The old quartic "pure IC" enumerated the subgroup (T1). A real pipeline (line relations with one large prime, Lanczos mod ell, descent through canonical Pic0 arithmetic) verified on 51 of 51 instances and costs IC/rho = %.3f [%.2f, %.2f] at q = %d. Exponent on q >= 4051: IC %.3f [%.3f, %.3f], rho %.3f [%.3f, %.3f], gap %.3f [%.3f, %.3f] (w = 0). Lead N4 stays open by the frozen rule. Scope: GHS-weak family only, plain rho, shared VM.\n'%(B[-1]['ratio'][0],B[-1]['ratio_lo'][0],B[-1]['ratio_hi'][0],B[-1]['q'],*pf['ic_exp_w0'],*pf['rho_exp_w0'],*pf['gap_w0']))
t.append('== Cold single-target cost\n')
t.append('#table(columns: 8, align: right, [q],[units ok],[rho runs],[IC-B0 M-eq],[rho M-eq],[IC/rho w=0 (95%)],[IC/rho w=1 (95%)],[resolved],\n')
for r in B:
    t.append('[%d],[%d/%d],[%d],[%s],[%s],[%.2f (%.2f-%.2f)],[%.2f (%.2f-%.2f)],[%s],\n'%(r['q'],r['units_ok'],r['units'],r['rho_runs'],f3(r['ic_meq'][0]),f3(r['rho_meq'][0]),r['ratio'][0],r['ratio_lo'][0],r['ratio_hi'][0],r['ratio'][2],r['ratio_lo'][2],r['ratio_hi'][2],r['resolved']))
t.append(')\n')
t.append('Baseline arm IC-A0r (all rational points, retries charged): '+'; '.join('q=%d: %s M-eq'%(r['q'],f3(r['ic_meq'][0])) for r in A)+'.\n')
t.append('#figure(image("reports/cost_vs_q.png", width: 82%), caption: [Cold cost vs q (recorded means; w = 1 GAE dotted). Source: results/summary_all.json, grid commit in README.])\n')
t.append('#figure(image("reports/ratio_vs_q.png", width: 78%), caption: [IC-B0 / rho with 95% bootstrap intervals (2000 resamples of units and runs).])\n')
t.append('#figure(image("reports/phases.png", width: 78%), caption: [Phase shares of IC-B0 cost: linear algebra and relations dominate for q >= 4051; descent (the only phase that adds Pic classes) falls to 1%.])\n')
t.append('== Exponents\n')
t.append('#table(columns: 5, align: left, [range],[IC-B0 (w=0)],[rho (w=0)],[gap (w=0)],[gap (w=1)],\n')
fa=fits['B0']
t.append('[all 11 sizes],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],\n'%(*fa['ic_exp_w0'],*fa['rho_exp_w0'],*fa['gap_w0'],*fa['gap_w1']))
t.append('[q >= 4051 (primary)],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],\n'%(*pf['ic_exp_w0'],*pf['rho_exp_w0'],*pf['gap_w0'],*pf['gap_w1']))
fb=fits['A0r']
t.append('[IC-A0r, q <= 4051],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],\n'%(*fb['ic_exp_w0'],*fb['rho_exp_w0'],*fb['gap_w0'],*fb['gap_w1']))
t.append(')\n')
t.append('== T1: old quartic_ic.rs solves at q = 5, 7, 11\n')
t.append('#table(columns: 6, align: right, [q],[old S before targets],[loop over n = 1..ell share],[line relations (rank/cols)],[deg-3 pic_canon merges],[old / canonical targets correct],\n[5],[650 443],[99.82%],[0 (0/6)],[1],[10/10, 5 (5 n/a)],\n[7],[2 751 342],[99.95%],[1 (1/8)],[7],[10/10, 7 (3 n/a)],\n[11],[37 920 004],[99.99%],[3 (3/12)],[20],[10/10, 10]\n)\n')
t.append('The recovered scalars and the deg-6 table survive; the "recovery:ic" label, the deg-3 reduction, `2lp` (= `full`) and the rho S = 0 do not.\n')
t.append('== T3 and T4\n')
t.append('Addition cost (3523M + 44I, #2227) vs published 148M + 15SQ + 2I (Flon, Oyono, Ritzenthaler, flex, char > 5; not implemented): descent share of IC-B0 is 55% at q = 223, 13% at 4051, 1% at 262111; with descent free the ratio at 4051 moves 0.735 to 0.637, at 262111 0.122 to 0.120. Not the bottleneck where IC wins; linear algebra is (57 to 74%). Stop rule: gap interval above 0 at w = 0 and w = 1, so N4 is open.\n')
t.append('== Limits\n')
t.append('GHS-weak family (4 divides the order of E0, E0/F_{q^3}), plain rho without negation (sqrt 2 sensitivity), one shared 4-core VM at L0, wall time is a practicality note only (IC 0.57 of rho at q = 262111, count ratio 0.12), runs not bit-reproducible (HashMap order), arm A0 as frozen failed 5 of 15 units (A0r added post hoc), not compared with an external implementation.\n')
txt=''.join(t)
parts=txt.split('`')
for i in range(0,len(parts),2):
    parts[i]=parts[i].replace('_','\\_')
out='`'.join(parts)
import re
out=re.sub(r'image\("([^"]*)"', lambda m: 'image("'+m.group(1).replace('\\_','_')+'"', out)
open('REPORT.typ','w').write(out)
