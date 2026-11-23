from pathlib import Path
from datetime import datetime,timezone
import json,hashlib,typst,fitz
D=Path(__file__).resolve().parent;E=D.parent.parent
launch=json.loads((E/'launch.json').read_text());post=json.loads((E/'postprocess-launch.json').read_text())
assert (Path('/proc')/str(launch['producer_pid'])/'stat').read_text().split()[21]==launch['start_ticks']
assert (Path('/proc')/str(post['postprocessor_pid'])/'stat').read_text().split()[21]==post['start_ticks']
assert not (E/'exit-code.txt').exists()
size=(E/'runs/cases.jsonl').stat().st_size;rows=[];h=hashlib.sha256();prefix=0
with (E/'runs/cases.jsonl').open('rb') as f:
 while f.tell()<size:
  line=f.readline(size-f.tell())
  if not line.endswith(b'\n'):break
  z=json.loads(line);assert z['status']=='verified';rows.append(z);h.update(line);prefix+=len(line)
out=dict(observed_at=datetime.now(timezone.utc).isoformat(),status='live_prefix_observation',completed_context_rows=len(rows),planned_context_rows=1698,products=sum(z['products'] for z in rows),mutations_rejected=sum(z['mutations_rejected'] for z in rows),equal_wrap_rejections=sum(z['equal_wrap_rejections'] for z in rows),radix_guard_rejections=sum(z['radix_guard_rejections'] for z in rows),complete_prefix_bytes=prefix,complete_prefix_sha256=h.hexdigest(),producer_pid=launch['producer_pid'],producer_start_ticks=launch['start_ticks'],producer_session=post['producer_session'],postprocessor_pid=post['postprocessor_pid'],postprocessor_session=post['postprocessor_session'],terminal_result=False,restarted=False)
(D/'snapshot.json').write_text(json.dumps(out,indent=2)+'\n')
svg=f'<svg xmlns="http://www.w3.org/2000/svg" width="760" height="140"><rect width="760" height="140" fill="white"/><g font-family="sans-serif" font-size="17"><text x="10" y="28">Complete context receipts at snapshot: {len(rows)} / 1698</text><rect x="10" y="45" width="700" height="35" fill="#dedede"/><rect x="10" y="45" width="{700*len(rows)/1698}" height="35" fill="#286e89"/><text x="10" y="117">Live run; full acceptance and A/A remain pending.</text></g></svg>'
(D/'coverage.svg').write_text(svg)
t=f'''#set page(paper:"a4",margin:18mm)
#set text(size:11pt)
= Fused companion validation: launch observation
{out['observed_at']} · Live process snapshot; not a terminal report

The single frozen invocation is running with its original 3600-second cap.
Producer PID {launch['producer_pid']} and follower PID {post['postprocessor_pid']}
were verified against their process start ticks at this observation. The
follower never launches trials; it runs analysis and export checking only
after producer exit zero. No restart occurred.

#image("coverage.svg",width:100%)

*Completed prefix.* {len(rows)} context rows, {out['products']} product calls,
{out['mutations_rejected']} rejected output/count/trace/sign mutations,
{out['equal_wrap_rejections']} equal-wrap rejections and
{out['radix_guard_rejections']} radix-guard rejections. These counts describe
only complete rows at this timestamp. Later contexts, carry cohort and A/A
are not inferred from this prefix.

*Construction under test.* Deferred-reduction companion application retains
positive packed coefficients through convolution, extraction, wrap, sum and
inverse scaling, decoding and reducing the final n coefficients. The radix
bound, every carry-fixture intermediate, archived outputs and independent
native bit-volume accounting are checked under the frozen contract.

*Full declared cohort.* All369 archived contexts per arm, plus480 carry
contexts per arm: 1698 context rows and34248 product calls. Every archived
full, inverse and principal-block product is consumed exactly once per arm.
The archive supplies a dense-verified correctness baseline; no timing
comparison is made. Field counts, native arithmetic/bit volumes and bytes
remain separate.

*Predata evidence.* Commit3e77b2c; protocol hash
21e18b0471cccf698b0ffc2f82e02fe3e393f86004813af3d41a145aa1a0c843.
The schedule preflight matched all1698cells before data. Ten source files
and two public archives are hash-locked. Source and analyzer were committed
before the first trial. The primitive actual-overflow certificate is separate
from rejecting a radix that lacks the proof bound.

*Custody.* Producer session {post['producer_session']}; follower session
{post['postprocessor_session']}. Native timeout deadline epoch
{launch['deadline_epoch']}. Follow these handles; never rearm the timeout.
The original capped Schur panel remains unchanged. No external jobs changed.

*Snapshot integrity.* {prefix} complete-prefix bytes, SHA256
{h.hexdigest()}. snapshot.json retains exact fields. Raw output is mutable
while the producer runs and is not sealed by this launch report. Final
rendering is prepared but requires successful producer, analyzer, export and
follower terminal receipts. Freeze and launch records remain independent of
any later result.
'''
(D/'report.typ').write_text(t)
typst.compile(str(D/'report.typ'),output=str(D/'report.pdf'));doc=fitz.open(D/'report.pdf')
for i,page in enumerate(doc):page.get_pixmap(matrix=fitz.Matrix(1.3,1.3)).save(str(D/f'page-{i+1}.png'))
print(json.dumps(out));print('pages',len(doc))
