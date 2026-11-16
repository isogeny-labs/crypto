from pathlib import Path
import json,hashlib,subprocess
D=Path(__file__).parent
R=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029')
B=R/'research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets'
names=['mumford-normalization-transport','mumford-normalization-validity','affine-coefficient-loop','word-coefficient-update','wide-modpow-inversion','coefficient-array-storage','coefficient-traversal','traversal-polynomial-composition','scale-trim-storage','normalization-source-correction','stored-divisor-validity','normalization-inverse-support','normalization-multiplicity']
# Each joint has one proposed role; none is assigned or judged by this producer.
joints=[
('source','Source identity','canonical choose/transform/map_divisor and related collector bodies','Try the known wrong first-61-line gate excerpt; demand exact named ranges and explicit identifier changes.','Independently extract each named body and reconstruct call order.'),
('arithmetic','Word arithmetic','small powers, binomial recurrence, wide modular product and inversion','Use p=0,1, composite p, noncanonical residues, a=0, k>5 and maximal words; identify excluded domains before applying lemmas.','Derive each intermediate bound and inverse premise directly from Rust operations.'),
('traversal','Traversal and storage','nested immutable coefficient / mutable slot traversal, trim and pad','Drop a row, change iteration order, alias coefficient with output, insert an internal zero, exceed capacity.','Derive the coefficient sum and prove exact slice lengths from source degree constraints.'),
('metadata','Representation premises','degree metadata, monicity, canonical inputs and ignored array tails','Supply degree metadata inconsistent with natDegree, degree>2, or nonzero unused tails; check what is assumed and what source actually validates.','Trace constructors/callers that establish each premise; do not infer validity from array length.'),
('identity','Degree-zero branch','source returns original divisor unchanged; model proves decoded identity validity','Keep arbitrary unused storage words with degree0; distinguish byte identity from decoded polynomial identity.','Derive u=1,v=0 from the precise validity predicate and degree premise.'),
('validity','Mumford predicate','monic u, degree bound, ordinate bound and divisibility','Use singular f, a=0 and nonmonic u; determine whether any geometric claim was smuggled into polynomial validity.','Derive scaling powers d,3,6 and transport each of the four clauses.'),
('inverse','Inverse and sign','polynomial round trips and affine signed-support equivalence','Reverse direction, negate beta incorrectly, omit y scaling, or discard sign; verify which controls detect each mutation.','Derive inverse coordinate map from x_old=a*x_new+b and y_old=a^3*y_new.'),
('multiplicity','Multiplicity','all-power divisibility and exact finite polynomial multiplicity','Use zero polynomial, k=0, repeated root, ramified curve point; reject any inference equating polynomial and curve valuation without a bridge.','Derive quotient transport and both directions using the inverse; state valuation obligations separately.'),
('trust','Proof execution and dependencies','native checks, axiom audit, compiled cache trust boundary','Alter imported helper after receipt; omit a module; use prebuilt olean with mismatched source.','Rebuild exact named modules in an isolated immutable tree and perform fresh axiom readback.'),
('geometry','Geometric scope boundary','remaining local valuations, infinity and principal-divisor transport','Attempt to claim a Jacobian group isomorphism from rational-point tests or polynomial validity alone.','Construct the curve/function-field map, extend it at infinity and derive local orders before group transport.')]
records=[{'id':i,'joint':j,'claim':c,'proposed_reviewer_role':'independent-'+i,'assigned_reviewer':None,'worked_attack':a,'blind_rederivation':b,'status':'awaiting_independent_review'} for i,j,c,a,b in joints]
inputs=[];checks=[]
for name in names:
 p=B/name
 assert p.is_dir(),name
 custody=json.loads((p/'custody-map.json').read_text());included=[x for x in custody['files'] if x['included_path']]
 for x in included:assert hashlib.sha256((p/x['included_path']).read_bytes()).hexdigest()==x['sha256'],(name,x['included_path'])
 checks.append({'packet':name,'included_files_verified':len(included),'custody_sha256':hashlib.sha256((p/'custody-map.json').read_bytes()).hexdigest(),'scope':'included bytes only; excluded build artifacts not rechecked'})
 for f in sorted(p.iterdir()):
  if f.is_file() and (f.name in ['status.json','custody-map.json','check.py','source-map.json','PLAN.md'] or f.name.endswith('.lean.txt')):inputs.append(f)
inputs += [R/'research/five_leads_e2e_20261019/native/genus2_model_gate.rs',R/'src/cryptanalysis/three_leads.rs']
rel=[str(f.relative_to(R)) for f in inputs]
subprocess.run(['git','diff','--cached','--quiet','HEAD','--',*rel],cwd=R,check=True)
tracked=subprocess.check_output(['git','ls-files','-s','--',*rel],cwd=R,text=True)
index={line.split('\t',1)[1]:line.split()[1] for line in tracked.splitlines()}
rows=[]
for f,r in zip(inputs,rel):
 data=f.read_bytes();blob=hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest()
 assert index.get(r)==blob,('index mismatch',r)
 rows.append({'path':r,'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest(),'git_blob':blob})
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=R,text=True).strip()
(D/'bindings.json').write_text(json.dumps({'producer_head':head,'inputs':rows,'verification':'working bytes match Git index blobs and selected index entries match HEAD; custody bytes checked; no fresh Lean replay'},indent=2)+'\n')
(D/'review-plan.json').write_text(json.dumps({'schema':'research.source-fidelity-review-contract/v1','task':'T-1187','producer_prior':'Exact native statements have producer checks; independent source fidelity not established.','producer_authored':True,'independent_verdict':None,'mutual_blindness':'Separate reviewer sessions; withhold other reviewers findings until independent reports are sealed.','joints':records,'required_outputs':['fresh native receipts and axiom audit','clause-by-clause source/model mapping','counterexample or explicit domain for each attack','independent report with precise scope'],'scientific_execution':False,'packet_custody_checks':checks},indent=2)+'\n')
(D/'status.json').write_text(json.dumps({'schema':'research.review-preparation-status/v1','producer_head':head,'packets_bound':len(names),'included_files_hash_checked':sum(x['included_files_verified'] for x in checks),'selected_index_bound_inputs':len(rows),'review_joints':len(records),'independent_reviews_completed':0,'fresh_proof_replays':0,'scientific_executions':0,'runtime_amendment':'unreviewed/unintegrated; prior failed qualification preserved','campaign_deadline':'2026-10-31T06:00:00Z','campaign_renewed':False},indent=2)+'\n')
text='# Normalization source-fidelity review contract\n\nProducer-organized review preparation, not an independent verdict. Thirteen packets are hash-bound to the recorded producer head; included custody bytes were rechecked. No proof replay or scientific execution is claimed.\n\n'
for x in records:text+=f"## {x['joint']}\n\nClaim: {x['claim']}.\n\nAttack: {x['worked_attack']}\n\nBlind re-derivation: {x['blind_rederivation']}\n\nOwner role: {x['proposed_reviewer_role']}; unassigned.\n\n"
text+='Review procedure: separate reviewer sessions, immutable pinned inputs, fresh named-module builds and axiom readback, exact premise mapping and explicit trust boundaries. Sources: review-evidence/references/lean-review.md, corrected source extraction and prior Mathlib affine-substitution references. Runtime: mount-diff proposal remains unreviewed and unintegrated; original qualification remains failed. Campaign deadline remains expired.\n'
(D/'REPORT.md').write_text(text)
print(json.loads((D/'status.json').read_text()))
