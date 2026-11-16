# Presentation, archival metadata and hashes only. All solver mathematics is native.
import json,hashlib,shutil
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from reportlab.platypus import SimpleDocTemplate,Paragraph,Spacer,Table,PageBreak,Image
from reportlab.lib.styles import getSampleStyleSheet
out=Path(__file__).parent;root=out.parents[1]
ids=['pair-cell-independent-binary-odd-oracles-20261019-v1','pair-cell-unconditioned-n83-profile-20261019-v1','guarded-chord-trace-original-hard-n83-d18-20261019-v1','pair-cell-fixed-sum-n83-profile-20261019-v1','adaptive-dual-nullspace-pair-cells-n83-20261019-v2']
runs=[]
for eid in ids:
 src=root/'lab/ledger/attempts'/eid/'0001';dst=out/'raw'/eid;dst.mkdir(parents=True,exist_ok=False)
 for f in src.iterdir():
  if f.is_file():shutil.copyfile(f,dst/f.name)
 r=json.loads((src/'scientific_result.json').read_text());p=json.loads((src/'process.json').read_text());runs.append(dict(id=eid,status=r['status'],wall_seconds=p['wall_seconds'],peak_process_rss_kib=p['peak_process_rss_kib'],result=r))
fixed=runs[-2]['result']['evidence'];adaptive=runs[-1]['result']['evidence'];coverage=json.loads((root/'research/fixed_pair_cell_profile_20261019_v1/COVERAGE.json').read_text())
report=dict(schema='rootfinding-cycle-report/v1',hypothesis='Condition the pair sum, then adapt whole-cell directions to an empty point dual nullspace.',experiments=runs,coverage=coverage,component_tuple_count=len(fixed['verified_component_pair_tuples']),scope='Binary pair component only; all original five-source roots remain unresolved; no qualified speed ratio.',gate_passed=False,novelty='Classical duality and linearization; no priority claim',next_experiment='Deduplicate adaptive affine cells and measure residual (p,s) coverage with exact affine-subspace intersections; preregister controller overhead and sound original-chain coverage before another hard search.')
(out/'REPORT.json').write_text(json.dumps(report,indent=2)+'\n')
text=f'''# Exact fixed-sum and adaptive pair cells

The fixed-sum hypothesis passed its declared calibration. On GF(2^83), d18, {fixed['positive_dimension_cells_certified_empty']} positive-dimensional cells were certified empty among {fixed['cells_tested']} configurations. All {fixed['point_fibers_verified']} point-fiber calls recovered complete ordered pair sets. The corpus has {coverage['distinct_p_s']} distinct (p,s) points, of which {coverage['distinct_certified_empty_p_s']} belong to certified empty cells, and {len(fixed['verified_component_pair_tuples'])} distinct recovered (x,y,s) tuples. Overlapping configurations are not added as coverage.

Adaptive directions change the tested assumption: instead of arbitrary low-coordinate directions, construct the nullspace of the point certificate. Registered exposed-training replication produced {adaptive['wide_certified_cells']} empty affine cells, each of dimension {adaptive['minimum_dimension']} (maximum {adaptive['maximum_dimension']}). Each cell contains 2^65 auxiliary values for its fixed p, proved by a full basis and dual replay. Their volumes are not a union. Even their volume sum is at most2^69, a2^-32 fraction of the full2^101 (p,s) grid; this is an upper bound, not observed global coverage. The independent exhaustive small oracle checked {adaptive['small_member_checks']} cell members across {adaptive['small_certified_cells']} certificates over GF8/GF16/GF32/GF128.

## Output contract and proof

For p=x+y in D, q_i=e_i^2+p e_i and S3=0 is exactly sum x_i(q_i^2+s q_i)=1+s^2 p^2. Recovering the full binary kernel recovers every ordered pair, including zero and repeated sources. An empty point dual lambda defines V by lambda(v q_i)=0 and lambda(v^2 p^2)=0. Thus the same lambda certifies every s in s0+V empty. The constraint rank is at most d, including p0, so dim V>=n-d=65. See the native sources and PROOF.md for exact hypotheses and bounds. Odd characteristic has a separate two-quadratic chart, tested only on small fields; the binary proof does not transfer to odd fields.

## Evidence boundaries

The unconditioned predecessor had zero positive-dimensional certificates. The changed guarded chord-trace hard run returned UNKNOWN with zero witnesses; its raw result is retained. The new cell results certify the declared first-pair components, not the original339-variable five-source hard system. The full outer grid is2^101 (p,s) pairs. A universal controller, all requested prime/extension and binary/Koblitz sizes, matched genuine full all-roots F4/F5/F6 baselines, novelty review and the immutable performance gate remain open. No qualified 2x,5x or8x end-to-end gain is claimed. Times in REPORT.json are single cold supervisor diagnostics on a shared host, including process setup and serialization; no A/A or isolation qualification. Training was exposed, not a holdout.

## Next bounded experiment

Deduplicate/intersect the emitted adaptive cells at each p and count the exact union by affine-subspace methods. Freeze a disjoint residual partition with a hard state/output cap; compare every small-field residual against exhaustive literal S3. Then test a coverage-preserving controller on partial original-chain states, paying all pair sums, other auxiliaries and final verification. Do not rerun the unchanged failed hard formula. Implement the separate odd two-quadratic chart over the NIST fields next; do not call the small odd checks full-width support.

## Requirements

| Requirement | Status |
|---|---|
| Exact declared binary pair fibers and empty cells | Verified calibration |
| Complete339-coordinate original hard roots | Unresolved |
| All prime, extension, binary and Koblitz families/sizes | Partial; wide new component only GF(2^83) |
| Genuine matched F4/F5/F6,20 hard inputs,3 cold repetitions | Not completed |
| Qualified>=2x floor; requested stronger gains | Not established |
| Novel algorithm priority | Not established |
| Research PR and paper addendum | This publication; no merge |
'''
(out/'REPORT.md').write_text(text)
(out/'NEXT_QUEUE.md').write_text(report['next_experiment']+'\n\nOdd route: implement q quadratic then X^2-pX+q root extraction over P192/P224/P256/P384/P521, retaining all zero-discriminant and domain branches.\n')
fig,ax=plt.subplots(figsize=(7,3));rows=coverage['by_dimension'];ax.bar([str(x['dimension']) for x in rows],[x['certified'] for x in rows]);ax.set(xlabel='Declared cell dimension',ylabel='Certified empty configurations',title='Fixed-sum calibration: 360 configurations per dimension',ylim=(0,380));fig.tight_layout();fig.savefig(out/'cell-certificates.svg');fig.savefig(out/'cell-certificates.png',dpi=150);plt.close(fig)
styles=getSampleStyleSheet();story=[]
for i,(title,body) in enumerate([('Exact pair-cell certificates',text.split('## Evidence boundaries')[0]),('Evidence, limits and next experiment','## Evidence boundaries'+text.split('## Evidence boundaries')[1])]):
 if i:story.append(PageBreak())
 story.append(Paragraph(title,styles['Title']))
 for para in body.split('\n\n'):
  if para.startswith('|'):continue
  story.append(Paragraph(para.replace('&','&amp;').replace('<','&lt;').replace('\n',' '),styles['BodyText']));story.append(Spacer(1,9))
 if i==0:story.append(Image(str(out/'cell-certificates.png'),width=450,height=193))
SimpleDocTemplate(str(out/'PAPER_ADDENDUM.pdf'),title='Exact Pair-Cell Certificates: Research Addendum',author='Anonymous research working draft').build(story)
(out/'REPRODUCE.md').write_text('''# Reproduce the component

Build each native C++ worker with g++ -O3 -std=c++17 -Wall -Wextra -Werror native.cpp -o native in its own source directory. Run pair_cell_obstruction worker with `replay controls`, `replay relaxed`, or `replay fixed`; adaptive_pair_cells takes only `replay`. Use one permitted CPU,4GiB and an external90s process-group deadline. These are correctness/profile replay commands, not qualified benchmark timings. Native guards include output serialization; only the parent supervisor charges final stdout lifetime. Frozen preregistrations and raw supervisor records are in raw/. Coordinator replay additionally needs the original anchored multi-GiB journal, intentionally excluded. Sources compile without external mathematical libraries. The original hard fixture and SAT dependencies are hash-identified in retained prior records but not included in this component publication. No claim that this subset reruns that hard search.\n''')
print(json.dumps({'runs':[(x['id'],x['status'],x['wall_seconds']) for x in runs],'adaptive':{k:v for k,v in adaptive.items() if k!='cells'}}))
