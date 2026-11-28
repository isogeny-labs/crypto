"""Render recorded Rust analysis; no new experimental statistics."""
from pathlib import Path
import json
import typst
study=Path(__file__).resolve().parent.parent
data=json.loads((study/'PROFILE_ANALYSIS_179.json').read_text())
colors={'compact-sparse':'#2763a0','compact-dag':'#c86a27'}
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="810" viewBox="0 0 1120 810"><rect width="1120" height="810" fill="white"/><g font-family="sans-serif" fill="#19304a"><text x="22" y="31" font-size="24">Child instruction attribution169: all36 fixed combinations</text><text x="22" y="58" font-size="16">Simulated Ir; one profile per bar. Complete saved proofs replayed. No runtime ratio or admission.</text>']
for col,p in enumerate([2147483647,9007199254740881]):
 for row,n in enumerate([3,4,6]):
  x=70+col*550;y=105+row*214;base=y+143
  svg.append(f'<text x="{x}" y="{y-12}" font-size="17">p={p}, n={n}</text>')
  for tick in [0,10,20,30]:
   yy=base-tick*4
   svg.append(f'<line x1="{x}" y1="{yy}" x2="{x+440}" y2="{yy}" stroke="#d5dfe7"/><text x="{x-9}" y="{yy+5}" text-anchor="end" font-size="13">{tick}</text>')
  for i,(form,k) in enumerate((f,k) for f in ['compact-sparse','compact-dag'] for k in ['frontier','packed-products','packed-batch']):
   r=next(c for c in data['cases'] if c['prime']==p and c['n']==n and c['form']==form and c['kernel']==k)
   total=r['raw_total_ir'];alloc=r['visible_allocator_self_ir'];assert 0<=alloc<=total<=30000000
   xx=x+12+i*72;h=total/1e6*4;ha=alloc/1e6*4
   svg.append(f'<rect x="{xx}" y="{base-h}" width="47" height="{h}" fill="{colors[form]}"/><rect x="{xx}" y="{base-ha}" width="47" height="{ha}" fill="#173747"/>')
   label=('S' if form=='compact-sparse' else 'D')+{'frontier':'103','packed-products':'231','packed-batch':'487'}[k]
   svg.append(f'<text x="{xx+23}" y="{base+21}" text-anchor="middle" font-size="13">{label}</text>')
svg.append('<text x="22" y="755" font-size="16">Y: millions of simulated child instructions. Blue: compact sparse. Orange: compact DAG.</text><text x="22" y="780" font-size="15">Dark lower segment: recognized allocator self-Ir among the top25 functions; a lower bound.</text><text x="22" y="802" font-size="14">Fixed seed7 LAST-star; controller/process wrappers excluded. Data: PROFILE_ANALYSIS_179.json; unsigned.</text></g></svg>')
(study/'visual-179/child-instructions.svg').write_text('\n'.join(svg)+'\n')
text='''# Complete child instruction attribution169

All36 planned profiles completed, with unchanged worker/input/tool pins and matching complete bases. Rust analyzer179 matches each raw Callgrind summary and totals count, then replays every saved original/proof with separate checker arithmetic and binds basis/stat receipts. The isolation monitor flags zero contention samples. This is diagnostic instruction attribution, not runtime or15x evidence. [pcc:unverified]

![Every declared input, certificate form and kernel; no bars omitted.](visual-179/child-instructions.svg)

The roster is fixed before data: primes2147483647/9007199254740881, n3/4/6, seed7 LAST-star; frozen85 compact-sparse and frozen96 compact-DAG workers, each with kernels103/231/487. Source, protocol and pins were published in da0157df0c and remotely checked in PUBLICATION_174.json. No target selection, replacement call or fresh/reserved seed was used.

The host is AMD EPYC9V74, localCPU0, one thread, zero cloud. Valgrind3.24.0 provides simulated Ir; perf sampling and hardware PMU counters are unavailable. Every call retains its manifest, raw profile, annotated self/inclusive tables, proof, worker output and process status. Child30s/2GiB/16MiB, case180s/global600s limits were declared in PROFILE_PROTOCOL_169.md. The probe and monitor are retained. All evidence remains unsigned and same-host.

## Interpretation and next hypothesis

Generic polynomial/provenance axpy, exponent-vector construction and allocator functions appear in the recorded self hotspots. The visible allocator category counts only six named allocator/wrapper symbols present in the top25; it is a lower bound, not the whole allocation cost. Self instruction categories are separate from overlapping inclusive call paths. Instruction share does not measure elapsed-time share, cache latency or cycles.

The profile covers the child, including construction/internal checks, wire serialization, first complete replay and proof writes. Controller readback/second replay, process launch and wrappers remain in the agreed application-call measurement boundary but are outside child instruction attribution. Profile wall durations are excluded from all performance statistics. The earlier repeated full-boundary96 result is unchanged; no improvement is admitted. [pcc:unverified]

The next hypothesis is a complete packed polynomial/provenance representation through construction, with guarded exact field arithmetic, rather than storing only two extra local resultant nodes. HYPOTHESIS_PACKED_PIPELINE_182.md defines the candidate and its required controls. It is a proposed implementation, not a guaranteed15x route. Transport/serialization changes must be applied to matched controls too, and full-boundary measurement remains decisive. [pcc:unverified]

## Recorded instruction counts

Each row below is one profile, not a runtime sample. Allocator share is the recorded lower bound from the top25 self entries; no confidence interval is inferred. Sources: PROFILE_ANALYSIS_179.json and the complete raw archive.

|Prime|n|Form|Kernel|Raw child Ir|Visible allocator self %|
|---|---:|---|---|---:|---:|
'''
for r in data['cases']:
 text+=f"|{r['prime']}|{r['n']}|{r['form']}|{r['kernel']}|{r['raw_total_ir']}|{r['visible_allocator_self_pct']:.3f}|\n"
text+='''
## Reproduction and unrelated root gate

Check `profile-169-179.tar.xz.sha256` from the study directory and extract `profile-169-179.tar.xz` into a fresh directory. Build `src/bin/profile-analyze.rs` with the study manifest, then run `profile-analyze EXTRACTED/matrix EXTRACTED/monitor.jsonl NEW_REPORT.json` to a fresh JSON output. It validates raw/parsed instruction totals and performs all36 full saved-proof replays. Raw profiles and artifacts are published with SHA256/index metadata; nothing was excluded.

The actual-root36-case adapter qualification remains separate. Full root unit165 built7561 tests but its failing pass was deliberately interrupted after46 observed failures; it is not a full pass. All46 observed failures were rechecked from its frozen ELF. Subsequent focused actual-source180 tests pass120 after repairing the double square charge and legacy observer-ID compatibility; the full root gate remains incomplete. ROOT_UNIT_PARTIAL_OUTCOME_177.json and SOURCE_CHECKS_180.json retain the scopes. No old evidence, trust key, catalog or verifier gate was rewritten. [pcc:unverified]

Canonical timing/benchmark graphs and frozen96 records remain unchanged. This diagnostic does not add a throughput result or a leaderboard row. The15x goal remains active and unachieved. [pcc:unverified]
'''
(study/'PROFILE_REPORT_179.md').write_text(text)
# Typst document layout only, drawing the recorded table as tabular content.
lines=['#set page(paper: "a4", margin: 1.3cm)','#set text(font: "DejaVu Sans", size: 9pt)','#set par(leading: 0.45em, spacing: 0.8em)','#set heading(numbering: none)']
for para in text.split('\n\n'):
 para=para.strip()
 if not para:continue
 if para.startswith('!['):
  cap,asset=para[2:].split('](',1);lines+=['#image('+json.dumps(asset[:-1])+', width: 100%)','#text(size: 8pt, '+json.dumps(cap)+')','#parbreak()']
 elif para.startswith('|'):
  rs=[r.split('|')[1:-1] for r in para.splitlines() if not r.startswith('|---')];cells=[]
  for r in rs:cells.extend('#text(size: 7pt, '+json.dumps(x.strip())+')' for x in r)
  lines.append('#table(columns: (1.55fr, 0.35fr, 1.25fr, 1.2fr, 1fr, 0.9fr), inset: 3pt, '+', '.join('['+c+']' for c in cells)+')')
 elif para.startswith('#'):
  pre,body=para.split(' ',1);lines.append('#heading(level: '+str(len(pre))+', '+json.dumps(body)+')')
 else:lines+=['#text('+json.dumps(para.replace('`',''))+')','#parbreak()']
(study/'profile-report-179.typ').write_text('\n'.join(lines)+'\n')
typst.compile(str(study/'profile-report-179.typ'),output=str(study/'profile-report-179.pdf'),root=str(study))
