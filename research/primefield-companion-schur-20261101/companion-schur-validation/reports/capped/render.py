from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,typst,fitz
D=Path(__file__).resolve().parent;E=D.parent.parent;ROOT=E.parent
assert (E/'exit-code.txt').read_text().strip()=='124'
assert (E/'postprocessor-exit.txt').read_text().strip()=='124'
launch=json.loads((E/'launch.json').read_text());post=json.loads((E/'postprocess-launch.json').read_text())
for pid in [launch['producer_pid'],post['postprocessor_pid']]:assert not Path('/proc',str(pid)).exists()
rows=[];h=hashlib.sha256();prefix=hashlib.sha256();full_bytes=0;complete_bytes=0;partial_bytes=0
with (E/'runs/cases.jsonl').open('rb') as f:
 for raw in f:
  h.update(raw);full_bytes+=len(raw)
  if not raw.endswith(b'\n'):partial_bytes+=len(raw);continue
  z=json.loads(raw);prefix.update(raw);complete_bytes+=len(raw)
  assert z['status']=='verified'
  rows.append(dict(arm=z['arm'],instance_id=z['instance_id'],kind=z['kind'],outcome=z['outcome'],r=z['payload']['plan']['r'],cut=z['payload']['plan']['cut'],directional_checks=z['directional_checks'],mutations_rejected=z['mutations_rejected'],generation_counts=z['payload']['generation'].get('counts'),reference_oracle_work=z['payload']['generation'].get('reference_oracle_work'),started_at=z['started_at'],finished_at=z['finished_at']))
for name in ['source-lock.json','input-lock.json']:
 for p,v in json.loads((E/name).read_text()).items():assert hashlib.sha256((ROOT/p).read_bytes()).hexdigest()==v,p
valid=sum(z['outcome']=='schur_verified' for z in rows);obstructed=len(rows)-valid
out=dict(status='capped_incomplete_no_restart',observed_at=datetime.now(timezone.utc).isoformat(),producer_exit=124,postprocessor_exit=124,planned_context_rows=642,completed_context_rows=len(rows),without_completion_receipt=642-len(rows),attempted_context_rows=None,attempted_scope='Completed rows known; no start receipt for unfinished case',valid_schur_rows=valid,leading_obstruction_rows=obstructed,directional_checks=sum(z['directional_checks'] for z in rows),mutations_rejected=sum(z['mutations_rejected'] for z in rows),aa_completed=False,synthetic_rows_completed=sum(z['kind']=='synthetic' for z in rows),full_panel_export_generated=False,source_locks_verified=11,input_locks_verified=1,producer_and_postprocessor_absent=True,restarted=False,raw_bytes=full_bytes,raw_sha256=h.hexdigest(),complete_prefix_bytes=complete_bytes,complete_prefix_sha256=prefix.hexdigest(),partial_trailing_bytes=partial_bytes,rows=rows)
(E/'terminal-reconciliation.json').write_text(json.dumps(out,indent=2)+'\n')
(D/'chart-data.json').write_text(json.dumps(dict(completed=len(rows),valid=valid,obstructed=obstructed,without_receipt=642-len(rows)),indent=2)+'\n')
svg=f'''<svg xmlns="http://www.w3.org/2000/svg" width="760" height="175"><rect width="760" height="175" fill="white"/><g font-family="sans-serif" font-size="16"><text x="10" y="25">Complete Schur context receipts: {len(rows)} of 642</text><rect x="10" y="45" width="{700*len(rows)/642}" height="35" fill="#286e89"/><rect x="{10+700*len(rows)/642}" y="45" width="{700*(642-len(rows))/642}" height="35" fill="#dedede"/><text x="10" y="111">{valid} verified constructions; {obstructed} leading-block obstructions</text><text x="10" y="147">{642-len(rows)} planned rows have no completion receipt. No full-panel verdict.</text></g></svg>'''
(D/'coverage.svg').write_text(svg)
examples=[z for z in rows if z['instance_id'] in ['18','27','36']]
lines=[]
for z in examples:
 if z['generation_counts']:
  c=z['generation_counts'];o=z['reference_oracle_work']
  lines.append(f'- Source {z["instance_id"]}, r={z["r"]}, cut={z["cut"]}: {c["field_products"]:,} field products, {c["integer_multiplications"]:,} native integer products, {c["packing_bytes"]:,} packing bytes; {o["forward_calls"]+o["transpose_calls"]} reference leading solves and {o["dense_terms"]:,} dense reference terms.')
 else:lines.append(f'- Source {z["instance_id"]}, r={z["r"]}: certified leading-block obstruction; no complete Schur generator.')
text='\n'.join(lines)
t=f'''#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Schur panel: original cap reached
Terminal incomplete-run report · {out['observed_at']}

The single Schur invocation reached its original 3,600-second cap and returned
124. Its postprocessor recorded the same terminal result and stopped before
analysis/export. Both process handles are absent. No restart or budget
extension occurred. The frozen complete cohort remains 642 context rows.

#image("coverage.svg",width:100%)
Counts derive from every complete raw JSON line. The terminal reconciliation
retains full and prefix hashes, byte counts, row IDs and all observed branches.

*Retained evidence.* {len(rows)} completed context rows contain {valid} checked
Schur constructions and {obstructed} certified leading-block obstructions,
with {out['directional_checks']} directional product checks and
{out['mutations_rejected']} rejected corruption controls. These are observations
on the completed prefix. A/A and the synthetic control cohort did not complete.
The planned 96 invertible-full/blocked-leading controls are not claimed as
executed. The frozen analyzer was not forced onto this incomplete archive.

*Coverage and custody.* {642-len(rows)} planned rows lack a completion receipt;
the exact number of started but unfinished contexts is not recorded. All
{full_bytes:,} raw bytes remain, including {partial_bytes} trailing partial bytes.
Eleven frozen source locks and the public input lock reverified. Predata
f5cac20; protocol7403509a477e478f03427db64fd4c864e8e62ea99dba3f5d84f8bcd249204480.
The full source, log, native descriptors and original deadline receipts are
preserved. Previous launch reports remain historical observations.

*Completed construction examples.* These are generator-construction costs;
source application work and supplied reference-inverse work remain separate.

{text}

*Next technical step.* The per-column construction invokes the full source
operator 2s+6 times. The separate companion-fused candidate defers polynomial
coefficient reduction through extraction, wrapping and accumulation. Its
larger radix and extra native arithmetic must be checked and charged before
it can replace a source operator. That candidate remains unexecuted at this
report's preparation. No change is made to this capped instrument.

*Files.* terminal-reconciliation.json; producer.log; exit-code.txt;
postprocess-state.json; postprocessor-exit.txt; runs/cases.jsonl; source-lock.json;
input-lock.json; launch.json; protocol.json. Chart data, editable report source
and renderer accompany this PDF. No full-panel export or solver improvement
is inferred from the retained prefix.
'''
t=t.replace('protocol740','protocol 740').replace('_',chr(92)+'_');(D/'report.typ').write_text(t)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'));doc=fitz.open(D/'report.pdf')
for i,p in enumerate(doc):p.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(str(D/f'page-{i+1}.png'))
print(json.dumps({k:v for k,v in out.items() if k!='rows'},indent=2));print('pages',len(doc))
