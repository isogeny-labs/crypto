# Lays out recorded values (results/summary_all.json, summary_primary.json) into REPORT.typ. No computation.
import json, re
rows=[json.loads(l) for l in open('results/summary_all.json')]
P=[json.loads(l) for l in open('results/summary_primary.json')]
sz=lambda a: sorted([r for r in rows if r.get('arm')==a and 'ic_meq' in r],key=lambda r:r['q'])
B2,B1,B0n,B0=sz('B2'),sz('B1'),sz('B0n'),sz('B0')
cmp_=[r for r in rows if 'cmp' in r]
fit=lambda arr,a: [r for r in arr if r.get('fit')==a][0]
pf2,pf0=fit(P,'B2'),fit(P,'B0'); af2=fit(rows,'B2')
f3=lambda x:'%.3g'%x
t=['#set page(margin: 1.6cm)\n#set text(size: 9.5pt)\n#set heading(numbering: "1.")\n']
t.append('= N4 iteration 2: double large primes (B2) and structured elimination (B1) vs the recorded B0\n')
t.append('2026-10-21. Branch `claude/quartic-ic-n4-la-20261021`, protocol commit eab1bdfacd, records `results/`. Descriptive run report; interpretation in `README.md`. Cost in F_q-multiplication equivalents (M-eq); every IC unit verified kG = Q.\n')
t.append('== Result in brief\n')
t.append('B2 verified on 51 of 51 units (first attempts retried and charged) and costs IC/rho = %.3f at q = %d. Exponent on q >= 4051: B2 %.3f [%.3f, %.3f], rho %.3f [%.3f, %.3f]; the recorded B0 baseline had %.3f. B1 (structured elimination) is a small stage gain and a regression at q = 262111. B2 is the new baseline. GHS-weak family only; plain rho; shared VM.\n'%(B2[-1]['ratio'][0],B2[-1]['q'],*pf2['ic_exp_w0'],*pf2['rho_exp_w0'],pf0['ic_exp_w0'][0]))
t.append('== B2 against the matched rho\n')
t.append('#table(columns: 7, align: right, [q],[units ok],[rho runs],[B2 M-eq],[rho M-eq],[B2/rho w=0 (95%)],[resolved],\n')
for r in B2:
    t.append('[%d],[%d/%d],[%d],[%s],[%s],[%.3f (%.3f-%.3f)],[%s],\n'%(r['q'],r['units_ok'],r['units'],r['rho_runs'],f3(r['ic_meq'][0]),f3(r['rho_meq'][0]),r['ratio'][0],r['ratio_lo'][0],r['ratio_hi'][0],r['resolved']))
t.append(')\n')
t.append('#figure(image("reports/cost_vs_q.png", width: 82%), caption: [Cold cost vs q (recorded means). B0 and rho: records of PR 2283; B1, B2: this study.])\n')
t.append('#figure(image("reports/ratio_vs_b0.png", width: 78%), caption: [Cost ratios against the recorded B0 baseline, paired by size, 95% bootstrap over units.])\n')
t.append('#figure(image("reports/ratio_vs_rho.png", width: 78%), caption: [IC cost over matched rho cost with 95% bootstrap intervals.])\n')
t.append('#figure(image("reports/phases_b2.png", width: 78%), caption: [Phase shares of B2: linear algebra and relations; descent under 1% from q = 262111.])\n')
t.append('== Paired comparison with B0\n')
t.append('#table(columns: 5, align: right, [X/Y],[q],[X units],[Y units],[cost ratio (95%)],\n')
for r in sorted(cmp_,key=lambda r:(r['cmp'],r['q'])):
    t.append('[%s],[%d],[%d],[%d],[%.3f (%.3f-%.3f)],\n'%(r['cmp'],r['q'],r['nx'],r['ny'],r['ratio'],r['lo'],r['hi']))
t.append(')\n')
t.append('== Exponents\n')
t.append('#table(columns: 4, align: left, [range / arm],[IC (w=0)],[rho (w=0)],[gap (w=0)],\n[B2, q >= 4051],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],\n[B2, all 13 sizes],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],\n[B0 baseline, q >= 4051],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],[%.3f [%.3f, %.3f]],\n)\n'%(*pf2['ic_exp_w0'],*pf2['rho_exp_w0'],*pf2['gap_w0'],*af2['ic_exp_w0'],*af2['rho_exp_w0'],*af2['gap_w0'],*pf0['ic_exp_w0'],*pf0['rho_exp_w0'],*pf0['gap_w0']))
t.append('== Limits\n')
t.append('GHS-weak family only; plain rho without a negation map (up to sqrt 2); L0 shared VM; B2 first attempts at q = 223 included two wrong answers caught by the kG = Q check (vacuous hold-out at 30 columns); B1 regresses at q = 262111; not compared with an external implementation.\n')
txt=''.join(t)
parts=txt.split('`')
for i in range(0,len(parts),2): parts[i]=parts[i].replace('_','\\_')
out='`'.join(parts)
out=re.sub(r'image\("([^"]*)"', lambda m: 'image("'+m.group(1).replace('\\_','_')+'"', out)
open('REPORT.typ','w').write(out)
