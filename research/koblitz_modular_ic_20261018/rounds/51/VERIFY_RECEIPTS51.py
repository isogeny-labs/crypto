"""Verify receipt custody and historical Git source bytes; no math replay."""
from pathlib import Path
import hashlib,json,subprocess
h=Path(__file__).resolve().parent; repo=h.parents[3]
r=json.loads((h/'results.json').read_text())
assert all(x['status'] in ('accepted','rejected') for x in r['runs']), 'native work remains active'
paths={'diamond51':'source-data/diamond-input51.sing','glue-prep51':'source-data/glue-prep-input51.sing','glue-forward51':'source-data/glue-direction1-input51.sing','glue-reverse51':'source-data/glue-direction2-input51.sing','glue-reload51':'source-data/glue-reload-input51.sing','universal51':'UNIVERSAL51.sing','twist51':'TWIST51.sing','base23-51':'BASE23_51.sing','fiber51':'FIBER51.rs','overlap-lean51':'lean/Overlap51.lean'}
checks=[]
for row in r['runs']:
    p=h/'runs'/row['attempt']; rev=(p/'source-revision.txt').read_text().strip()
    key=next(k for k in paths if p.name.startswith(k+'-'))
    target='Overlap51.lean' if row['kind']=='Lean' else ('input.rs' if row['kind']=='Rust' else 'input.sing')
    runner='REPLAY_LEAN.sh' if row['kind']=='Lean' else ('REPLAY_FIBER.sh' if row['kind']=='Rust' else 'REPLAY_CAS.sh')
    for source,actual in [(paths[key],target),(runner,'runner.sh')]:
        full=str((h/source).relative_to(repo))
        blob=subprocess.check_output(['git','show',rev+':'+full],cwd=repo)
        assert blob==(p/actual).read_bytes(), (p.name,source,'historical Git byte mismatch')
        checks.append({'attempt':p.name,'revision':rev,'source':source,'sha256':hashlib.sha256(blob).hexdigest()})
    for line in (p/'source.sha256').read_text().splitlines():
        digest,filename=line.split('  ',1)
        assert hashlib.sha256((p/Path(filename).name).read_bytes()).hexdigest()==digest, (p.name,filename)
(h/'receipt-validation.json').write_text(json.dumps({'schema':'koblitz.receipt-custody/v1','round':51,'attempts':len(r['runs']),'historical_git_blob_checks':len(checks),'all_original_source_hashes_checked':True,'checks':checks,'scope':'source bytes and terminal receipts; not mathematical replay'},indent=2)+'\n')
print(json.dumps({'attempts':len(r['runs']),'historical_git_blob_checks':len(checks)}))
