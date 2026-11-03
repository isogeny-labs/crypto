"""Layout only: renders the recorded audit statements; computes no results."""
from pathlib import Path
from xml.sax.saxutils import escape
from reportlab.pdfgen import canvas
from reportlab.lib.colors import HexColor
from reportlab.lib.utils import simpleSplit

root = Path(__file__).resolve().parent
nodes = [
 (25, 25, 580, 48, 'One physical lane-iteration (already inside GPU timing)', '#dceaf5'),
 (25, 105, 270, 65, 'Cycle detection / seek / infinity', '#fbe6cc'),
 (335, 105, 270, 65, 'Eligible advance (escape double included)', '#dcefe6'),
 (335, 205, 270, 65, 'Test DP; emit and restart, or continue', '#dcefe6'),
]
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="630" height="310" viewBox="0 0 630 310">', '<rect width="630" height="310" fill="white"/>']
for x,y,w,h,label,fill in nodes:
 svg += [f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="8" fill="{fill}" stroke="#597080"/>', f'<text x="{x+w/2}" y="{y+h/2+5}" text-anchor="middle" font-family="sans-serif" font-size="12">{escape(label)}</text>']
svg += ['<path d="M160 73 V105 M470 73 V105 M470 170 V205" stroke="#36546b" stroke-width="2" fill="none"/>', '<text x="160" y="195" text-anchor="middle" font-family="sans-serif" font-size="12">No DP test on this path</text>', '<text x="315" y="298" text-anchor="middle" font-family="sans-serif" font-size="11">Physical attempts = advanced + cycle_detect + seek + infinity</text>', '</svg>']
(root/'accounting.svg').write_text('\n'.join(svg)+'\n')

c=canvas.Canvas(str(root/'REPORT.pdf'),pagesize=(595,842))
c.setTitle('BEST2 seeded-walk and replay audit - 2026-10-27')

def title(text,sub):
 c.setFillColor(HexColor('#15374d'));c.setFont('Helvetica-Bold',19);c.drawString(42,796,text)
 c.setFillColor(HexColor('#536672'));c.setFont('Helvetica',10);c.drawString(42,775,sub)

def para(text,y,size=10):
 c.setFillColor(HexColor('#263f50'));c.setFont('Helvetica',size)
 for line in simpleSplit(text,'Helvetica',size,510):
  c.drawString(42,y,line);y-=14
 return y-9

def section(text,y):
 c.setFillColor(HexColor('#15374d'));c.setFont('Helvetica-Bold',12);c.drawString(42,y,text);return y-20

def foot(n):
 c.setFillColor(HexColor('#536672'));c.setFont('Helvetica',8)
 c.drawString(42,28,'Source: seeded-audit-v1/README.md, PROTOCOL.md and checks/');c.drawRightString(552,28,str(n))

title('BEST2: validation remains open','27 October 2026 | PR #2689 | Accounting and correctness audit')
y=para('BEST2 stays disabled. The historical 0.947 estimate is derived from a standalone iteration model and GPU throughput. It is not a measured GPU solve-cost ratio or an end-to-end 95% confidence interval.',742,11)
y=section('Source findings and corrections',y-3)
for text in [
 'Timing repetitions reused identical starts and tables. The revised driver exposes reproducible seed namespaces and a fixed-table / varying-start control.',
 'The DP predicate and cycle hash both use limb 0. Score limb 1 being different does not establish independence. Cycle-detection and seek iterations skip DP testing.',
 'Warmup DPs were counted against a measured-only denominator. The driver now clears the counter after warmup; four warmup trajectory steps remain.',
 'Host seed replay could lose the restart number after an infinity start. The caller now receives the actual restart; a constructed cancellation-start regression passes.',
 'The standalone model uses hashed predicates, different escape mechanics and no restart after each DP. Its iteration ratio does not directly price the CUDA walk.'
]:y=para(text,y)
y=section('Count physical work and DP opportunities separately',y)
# Same node geometry and labels as the editable SVG above, scaled for the page.
scale=0.81; origin_x=42; top=y+5
for x,yy,w,h,label,fill in nodes:
 xx=origin_x+x*scale; bottom=top-(yy+h)*scale
 c.setFillColor(HexColor(fill));c.setStrokeColor(HexColor('#597080'));c.roundRect(xx,bottom,w*scale,h*scale,6,fill=1,stroke=1)
 c.setFillColor(HexColor('#263f50'));c.setFont('Helvetica',8)
 c.drawCentredString(xx+w*scale/2,bottom+h*scale/2-3,label)
c.setStrokeColor(HexColor('#36546b'))
for x,y1,y2 in [(160,73,105),(470,73,105),(470,170,205)]:c.line(origin_x+x*scale,top-y1*scale,origin_x+x*scale,top-y2*scale)
c.setFont('Helvetica',8);c.drawCentredString(origin_x+160*scale,top-195*scale,'No DP test on this path')
c.drawCentredString(297,top-298*scale,'Physical attempts = advanced + cycle_detect + seek + infinity')
foot(1);c.showPage()
title('Executed checks and remaining work','Small development tests establish correctness within their stated scope')
y=section('Checks actually executed',738)
for text in [
 'Both CPU field / curve / rho suites pass for BEST2=0 and BEST2=1. Runtime-W and compile-time-W equivalence now includes W=64.',
 'OpenSSL independently computes P+M and P-M on the named P-256 and P-384 curves: 2048 physical steps per arm, two namespaces, W=16, 32 iterations. Batch states and emitted DP identities match the per-phase trajectory.',
 'Toy scalar replay checks 4000 phases per arm, including subtraction, detection/seek and escape doubling. An infinity-start regression verifies matching host/device restart identities.',
 'The real Rust model source passes a dependency-free bookkeeping test: 12 seeds on both arms, correct synthetic group recoveries and identical replay counters. This is not a root integration build.',
 'The summary tool reproduces a synthetic eight-seed fixture ratio of 1.1 and rejects duplicated seeds. These fixtures are not measured DP distributions.'
]:y=para(text,y)
y=section('What has not been established',y-4)
for text in [
 'The DP bias-versus-deterministic-sampling diagnosis remains unresolved. No statistically powered exact-kernel seeded panel ran. Unit-test DP totals are too small for that inference.',
 'No authenticated GPU/EKS execution path, nvcc or device was available. No cloud job was submitted. Cargo 1.75 cannot parse the repository lockfile v4, so root build/session/replay evidence is missing.',
 'The new ecbench model uses hashed group keys and charges two full additions. It is explicitly separate from the coordinate-limb CUDA walk and cannot validate the NIST GPU cost estimate.',
 'A bounded known-answer GPU collision workload, GPU-to-host DP trace, independent scalar recovery and all-phase successful-collision accounting are still missing. The current RhoHost supports orders below 2^63 only.'
]:y=para(text,y)
y=section('Next discriminating experiments',y-4)
y=para('The frozen protocol retains W=16/32/64 and both curves, fixed-table and varied-table panels, paired seed-cluster bootstrap uncertainty, failures and raw hashes. Timing repetitions are controls, not new samples. Require independent replay and number recomputation before a benchmark row or default enablement.',y)
y=para('Primary-paper retrieval for Bernstein, Lange and Schwabe, eprint 2011/003, was blocked. This audit makes no paper-derived quantitative or novelty claim. See README.md for exact source paths, requirement status and blockers.',y,9)
foot(2);c.save()
