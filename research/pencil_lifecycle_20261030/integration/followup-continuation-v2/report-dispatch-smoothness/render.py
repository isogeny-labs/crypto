from pathlib import Path
import json,hashlib,datetime,textwrap,html,subprocess
D=Path(__file__).parent; R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_lifecycle_20261030/integration/followup-continuation-v2');h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
data=[
('Mixed dispatcher / field','pencil-mixed-dispatch-field-20261031-v1','pencil-mixed-dispatch-independent-review-20261031-v1','mixed-dispatch-field','mixed-dispatch-field-review','Original point/divisor polynomial validity and four nonzero weights over a field imply a valid nonzero-weight Some payload in every degree1+2 branch; explicit and nested None branches excluded.'),
('Mixed dispatcher / UInt64','pencil-mixed-dispatch-word-20261031-v1','pencil-mixed-dispatch-word-independent-review-20261031-v1','mixed-dispatch-word','mixed-dispatch-word-review','Prime 5<=p<2^31, canonical inputs/curve coefficients, n=1 and tables=None, four nonzero weights and original normalized validity imply canonical valid payload in literal source order.'),
('Repeated-support tripling / field','pencil-mixed-tripling-20261031-v1','pencil-tripling-independent-review-20261031-v1','mixed-tripling','mixed-tripling-review','Original mixed-input validity and the repeated positive-support guards establish nonzero output weights and polynomial validity for the literal tripling payload.'),
('Repeated-support tripling / UInt64','pencil-tripling-word-20261031-v1','pencil-tripling-word-independent-review-20261031-v1','mixed-tripling-word','mixed-tripling-word-review','Canonical input and prime bounds as above; raw constants 5,8,10 handled by explicit no-overflow/reduction correspondence, including p=5 and p=7. Scoped Some and validity composition.'),
('Degree2 equal-arm guard exclusion','pencil-degree22-guards-20261031-v1','pencil-degree22-guard-independent-review-20261031-v1','degree22-guards','degree22-guards-review','Over a field with 2!=0, crossed ordinate equality and earlier inverse guard false exclude the nested b=0,s=0 identity arm. Does not exclude direct-call identity or prove doubling output validity.'),
('Same-u extracted point','pencil-sameu-mixed-point-20261031-v1','pencil-same-u-extracted-independent-review-20261031-v1','sameu-mixed-point','sameu-mixed-point-review','Original two Mumford divisibilities, noninverse and nonzero slope difference give a valid extracted point. Four nonzero weights connect the exact field expression to normalized coordinates. Singular zero-ordinate example retained.'),
('Local nonsingularity bridge','pencil-sameu-local-smooth-20261031-v1','pencil-sameu-nonsingularity-independent-review-20261031-v1','sameu-local-smooth','sameu-local-smooth-review','Zero extracted ordinate forces a square factor and simultaneous value/derivative zero. Explicit local exclusion gives ordinate nonzero; with 2!=0 and four nonzero weights, subsequent doubling yields valid nonzero-weight output.'),
('Global coprimality bridge','pencil-sameu-global-smooth-20261031-v1','pencil-global-smooth-independent-review-20261031-v1','sameu-global-smooth','sameu-global-smooth-review','IsCoprime f f.derivative supplies Bezout exclusion at every x, then the local premise and nonzero extracted ordinate. Arbitrary field; characteristic-five controls pass; not a geometric characteristic-two smoothness assertion.')]
rows=[]
for name,p,r,pl,rl,claim in data:
 p=Path('/var/tmp')/p;r=Path('/var/tmp')/r
 rows.append(dict(component=name,status='reviewed_at_stated_scope',claim=claim,producer_path=str(p),producer_manifest_sha256=h(p/'manifest.json'),review_path=str(r),review_manifest_sha256=h(r/'manifest.json'),publication_producer='../packets/'+pl,publication_review='../packets/'+rl,cost_ratio=None))
S=R/'preflight-review/owner-lock-software-checks';software=[]
for label,d,expected in [('full library','full-lib','5793 passed; 0 failed; 128 ignored'),('affected GPU tests','gpu-touched','8 passed; 0 failed; 0 ignored')]:
 q=S/d;receipt=json.loads((q/'receipt.json').read_text());assert receipt['exit_code']==0;assert expected in (q/'stdout').read_text()
 software.append(dict(component=label,status='passed',result=expected,receipt_path=str(q/'receipt.json'),receipt_sha256=h(q/'receipt.json'),stdout_sha256=h(q/'stdout'),sources_unchanged=receipt['sources_unchanged']))
for label,d in [('maintained Clippy','maintained-clippy'),('final-source CLI build','ecbench-build')]:
 assert not (S/d/'receipt.json').exists();software.append(dict(component=label,status='pending_no_terminal_receipt',path=str(S/d)))
old=Path('/var/tmp/pencil-musl-native-provenance-review-20261031-v1');oldbinding={'path':str(old),'manifest_sha256':h(old/'manifest.json'),'scope':'Retained b985 static binary and static CLI checks; not final source qualification.'}
notes=['Formal results prove expression values and polynomial validity at their stated branch scope, not class sums, compiler execution, counter preservation or complete degree2+2 dispatch.','Producer and reviewer failed proof/import/numeral/timeout attempts remain in linked packets. Earlier GPU lock failures remain retained; later passing software receipts do not establish their source-level cause.','Reviewers disclose authorship of imported components. Kernel readback is accompanied by scoped source review; cached dependency objects were not all rebuilt.','All new full-cohort cost ratios are null; no scientific cohort was run for this snapshot. Existing scientific campaign limits are unchanged.']
obj={'schema':'research.component_snapshot/v1','utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'components':rows,'software':software,'retained_static_candidate':oldbinding,'limitations':notes,'new_full_cohort_cost_ratio':None,'scientific_trials':0}
(D/'index.json').write_text(json.dumps(obj,indent=2)+'\n')
lines=['Mixed-dispatch and nonsingularity component snapshot',obj['utc'],'']
for x in rows:lines += [x['component']+' - reviewed']+textwrap.wrap(x['claim'],108)+['']
lines+=['Software evidence']
for x in software:lines+=textwrap.wrap(x['component']+': '+x.get('result',x['status']),108)
lines+=['Retained b985 binary: reviewed static candidate; final-source build qualification remains separate.','']
for n in notes:lines+=textwrap.wrap(n,108)+['']
svg='<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="'+str(60+20*len(lines))+'"><rect width="100%" height="100%" fill="white"/>'
for i,l in enumerate(lines):svg+=f'<text x="30" y="{35+20*i}" font-family="sans-serif" font-size="15">{html.escape(l)}</text>'
svg+='</svg>';(D/'report.svg').write_text(svg);subprocess.run(['rsvg-convert','-f','pdf','-o',str(D/'report.pdf'),str(D/'report.svg')],check=True)
(D/'README.md').write_text('Neutral component snapshot. Exact producer/review manifests and prospective relative publication paths are in index.json. Global packet labels are prospective until root integration. Previous snapshots are unchanged.\n')
(D/'manifest.json').write_text(json.dumps({'schema':'artifact_manifest/v1','files':{p.name:h(p) for p in D.iterdir() if p.is_file() and p.name!='manifest.json'}},indent=2)+'\n');print(h(D/'manifest.json'))
