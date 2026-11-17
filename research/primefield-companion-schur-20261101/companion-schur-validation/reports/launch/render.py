from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,typst,fitz
D=Path(__file__).resolve().parent;E=D.parent.parent
launch=json.loads((E/'launch.json').read_text());pid=launch['producer_pid'];assert Path(f'/proc/{pid}/stat').read_text().split()[21]==launch['start_ticks']
rows=[];h=hashlib.sha256();size=0
with (E/'runs/cases.jsonl').open('rb') as f:
 for line in f:
  if not line.endswith(b'\n'):break
  z=json.loads(line);h.update(line);size+=len(line)
  rows.append(dict(arm=z['arm'],instance_id=z['instance_id'],kind=z['kind'],outcome=z['outcome'],directional_checks=z['directional_checks'],mutations_rejected=z['mutations_rejected']))
valid=sum(z['outcome']=='schur_verified' for z in rows);blocked=len(rows)-valid
snap=dict(observed_at=datetime.now(timezone.utc).isoformat(),producer_verified_live=True,producer_pid=pid,producer_session=launch['producer_session'],planned_contexts=642,complete_prefix_rows=len(rows),valid_schur_rows=valid,leading_obstruction_rows=blocked,checked_directions=sum(z['directional_checks'] for z in rows),rejected_mutations=sum(z['mutations_rejected'] for z in rows),prefix_bytes=size,prefix_sha256=h.hexdigest(),rows=rows,terminal_result=False)
(D/'snapshot.json').write_text(json.dumps(snap,indent=2)+'\n')
svg=f'''<svg xmlns="http://www.w3.org/2000/svg" width="720" height="150"><rect width="720" height="150" fill="white"/><g font-family="sans-serif" font-size="16"><text x="10" y="25">Complete prefix at the recorded observation: {len(rows)} of 642 contexts</text><rect x="10" y="45" width="{640*valid/max(1,len(rows))}" height="35" fill="#276b87"/><rect x="{10+640*valid/max(1,len(rows))}" y="45" width="{640*blocked/max(1,len(rows))}" height="35" fill="#b68429"/><text x="10" y="110">Blue: {valid} Schur identities; amber: {blocked} leading-block obstructions</text><text x="10" y="140">Both are checked branches. This is an incomplete running panel.</text></g></svg>'''
(D/'progress.svg').write_text(svg)
t=f'''#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Schur construction panel: launch and custody
Running experiment snapshot · {snap['observed_at']}

The frozen Schur panel is executing on its original producer. This observation
contains {len(rows)} complete context rows: {valid} checked Schur constructions
and {blocked} certified leading-block obstructions. These are prefix observations.
No complete-panel or export result exists at this snapshot. The postprocessor
follows this same invocation.

#image("progress.svg",width:100%)
Raw source: runs/cases.jsonl, first {size} bytes. The append-only prefix hash
and complete row IDs are in snapshot.json. This graph counts completed branches,
not statistical success estimates or a full recovery result.

*Full scope.* Each arm retains all 81 source contexts, using their balanced
leading block, and adds 240 synthetic contexts. Synthetic cases cross primes
7, 13, 19, 43; dimensions 2, 3, 4, 7, 12; cuts 1, floor(N/2), N-1 with
duplicates removed; and regular, zero-Schur, singular-Schur, invertible-full
with blocked-leading, and zero-matrix types. Two arms give 642 rows. All
leading obstructions remain in scope, including 96 synthetic rows whose
full matrix is invertible despite a singular leading block.

*Checks.* Exact reference elimination supplies a two-sided leading inverse
or a nonzero leading nullvector. With an inverse, check the full Schur
displacement, width s+2, all nested source and oracle counts, and both product
directions through the Schur wrapper and generated companion operator. With
a blocked inverse, generation and application must raise the specific
obstruction and retain partial work. That obstruction does not establish
full-matrix singularity. Equal-wrap, output, count and generator controls
are fixed before data.

*Custody.* Predata commit f5cac20. Protocol hash:
7403509a477e478f03427db64fd4c864e8e62ea99dba3f5d84f8bcd249204480.
All 642 execution-row seeds matched before data. Eleven source files and
one public input archive are locked. One fresh timeout 3600 invocation;
no restart or extension. Producer PID {pid}, session 68027; native timeout
PID 2104675. Postprocessor PID 2106367, session 68331. launch.json records
start ticks and a deadline estimate; the existing native timeout controls
the actual deadline.

*Costs.* Generator construction schedules 2s+6 full-source applications and
s+2 leading solves per direction. The reference inverse oracles record dense
dot-product terms and output reductions separately from source and native
work. Those calls are not free or evidence of fast inversion. No clock,
CPU or RSS comparison is claimed. All failures and partial rows are retained.

*Next action.* Follow the existing processes. After producer exit 0, the
postprocessor runs the frozen streaming analysis and full-file export check
once. A missing owner without an exit receipt is an interruption, not a reason
to rerun. After terminal checks, render and inspect a fresh final report and
seal all raw records.

#pagebreak()
= Derived next adapters: RHS and primal/dual lifting
Written and syntax-checked only. No adapter numerical trial ran in this round.
Source: companion-schur-lift/lift.py; proof: companion-schur-lift/DERIVATION.md.
These files do not alter the running Schur instrument.

For A=[[E,B],[C,D]] with E invertible, S=D-CE⁻¹B, and b=[b1;b2],
set h=b2-CE⁻¹b1. Given any trailing vector z, define
x=[E⁻¹(b1-Bz);z]. Then the exact residual is
Ax-b=[0;Sz-h]. Thus a checked Schur solution lifts to a full-system solution.

For any trailing v, define w=[-E⁻ᵗCᵗv;v]. Then
wᵗA=[0,vᵗS] and wᵗb=vᵗh. In particular, vᵗS=0 and vᵗh=1 produce
a normalized full-system exclusion witness. The existing separate feature
annihilation check remains necessary; a Gram-null identity alone does not
establish that feature-level condition.

Each adapter uses one full-source application and one leading-block solve.
Additional scalar work is m subtractions for RHS reduction and n for either
lift, where n=dim(E),m=dim(S). Dual lifting uses the transpose direction.
The helper records those calls through the Schur object's counters and lets
LeadingBlockObstruction propagate unchanged.

The next adapter gate must check both residual identities, normalized dual
witnesses, changed RHS/sign controls and complete nested costs, after the
current Schur construction panel is reconciled. Then a recursive backend
must replace the explicit leading-block reference oracles. No recursive
solver or new full ECDLP recovery is claimed by these unexecuted adapters.
'''
for a,b in [('All81','All 81'),('All642','All 642'),('Predataf5','Predata f5'),('protocol740','protocol 740'),('timeout3600','timeout 3600'),('ProducerPID','Producer PID '),('PostprocessorPID','Postprocessor PID '),('session68027','session 68027'),('session68331','session 68331'),('timeoutPID2104675','timeout PID 2104675')]:t=t.replace(a,b)
t=t.replace('_',chr(92)+'_');(D/'report.typ').write_text(t);typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'))
doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(str(D/f'page-{i+1}.png'))
print('pages',len(doc),'prefix_rows',len(rows),'valid',valid,'obstructed',blocked)
