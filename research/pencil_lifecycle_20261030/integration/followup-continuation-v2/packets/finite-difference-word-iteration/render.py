from pathlib import Path
import json,html,textwrap,subprocess
p=Path(__file__).resolve().parent;j=json.loads((p/'result.json').read_text());s=['<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="1300" viewBox="0 0 1000 1300"><rect width="1000" height="1300" fill="white"/><g font-family="sans-serif" fill="#183047">']
def text(x,y,t,size=18):s.append(f'<text x="{x}" y="{y}" font-size="{size}">{html.escape(t)}</text>')
def para(y,t):
 for z in textwrap.wrap(t,100):text(45,y,z,17);y+=25
 return y+15
text(45,50,'Canonical word recurrence: composed iteration',29)
text(45,84,'T-1525 | 2026-11-03 | producer checked; independent review pending',17)
y=para(125,'The new induction connects every UInt64 state to the previously checked ring recurrence. Canonicality is derived at each iteration. The resulting observation sequence equals direct polynomial evaluation in ZMod.')
for yy,title,body in [(y,'Canonical coefficients','p > 0; p <= 2^63; a,b,c,e,f < p'),(y+100,'Word iteration','(u,du,v) -> (add(u,du),add(du,dd),add(v,f))'),(y+200,'Decoded ring state','Quadratic U, first difference D, linear V at every n'),(y+300,'Ordered observations and final state','Old-state observations 0..p-1; final update at p retained')]:
 s.append(f'<rect x="45" y="{yy}" width="900" height="78" rx="8" fill="#edf3f7" stroke="#7092a8"/>');text(65,yy+29,title,21);text(65,yy+57,body,17)
y+=418
text(45,y,'Validation records',23);y+=36
y=para(y,'Attempt 1: two Lean invocations, 17.042756 seconds; Difference passed, WordIteration rejected one zero-cast type mismatch. Exact failed source and diagnostics are preserved.')
y=para(y,'Attempt 2: separately approved and frozen; five Lean invocations, 14.252913 seconds. Nine theorem declarations and eight controls passed 17 named axiom readbacks. The false final-update equality was rejected; three malformed axiom inventories were rejected.')
y=para(y,'Controls include constant padding, characteristic two, zero and late states, full wrap, retained final update, the p=2^63 boundary, and overflow outside that bound. All 28,797 dependency/toolchain file hashes match across the retained inventories.')
text(45,y,'Exact scope and next review',23);y+=36
y=para(y,'This proves the composed Lean word model, including ordered common-branch folds and canonical zero/equality correspondence. Source mapping binds the native capture-before-update order. Rust arrays, range iteration, compiler behavior and the remaining collector are separate refinement obligations.')
y=para(y,'No primality premise is needed for the recurrence. Canonical natural representatives are not a separately named theorem here. Existing full-sieve assumptions remain separate. No scientific trajectories or performance ratios were produced.')
y=para(y,'A different author will review the published exact theorem/source correspondence and replay within a prospective bound. JSON, CSV, source text, raw failure/success receipts and the archived logs preserve the evidence for that review.')
s.append('</g></svg>');assert y<1290,y;(p/'report.svg').write_text(''.join(s));subprocess.run(['rsvg-convert','-f','pdf','-o',str(p/'report.pdf'),str(p/'report.svg')],check=True)
