"""Recompute this software-control summary from retained successful receipts."""
from pathlib import Path
import base64, hashlib, json, re
p=Path(__file__).parent

def read(d,name):
    f=d/name
    return f.read_text() if f.exists() else base64.b64decode((d/(name+'.b64')).read_bytes()).decode()

checks=[q for q in p.glob('check-*') if q.is_dir() and (q/'exit').exists() and read(q,'exit').strip()=='0' and 'source_field' in read(q,'stdout')]
q=max(checks,key=lambda x:read(x,'end'))
rows=[json.loads(line.split('SIGNED_CONTROL ',1)[1]) for line in read(q,'stdout').splitlines() if 'SIGNED_CONTROL ' in line]
assert len(rows)==6 and sum(r['graph_prefixes_checked'] for r in rows)==364
native=[d for d in p.glob('native-check-*') if (d/'exit').exists() and read(d,'exit').strip()=='0']
n=max(native,key=lambda x:read(x,'end'))
assert read(q,'bindings.exit').strip()=='0' and read(n,'bindings.exit').strip()=='0'
assert read(q,'before.sha256')==read(n,'before.sha256')
(p/'controls.json').write_text(json.dumps({'schema':'software.signed-stream-controls/v1','scope':'one fixed known-answer target; six deterministic configurations; no performance inference','receipt':q.name,'rows':rows},indent=2)+'\n')
summary={'schema':'software.validation-summary/v1','parent_revision':'f7cb04ca6768a984ffb6dd1b5bc333ba5505215d','control_receipt':q.name,'native_receipt':n.name,'control_tests_passed':int(re.search(r'test result: ok\. (\d+) passed; 0 failed',read(q,'stdout')[read(q,'stdout').index('running 3 tests'):]).group(1)),'existing_native_tests_passed':int(re.search(r'test result: ok\. (\d+) passed; 0 failed',read(n,'stdout')).group(1)),'configurations':len(rows),'distinct_targets':len({json.dumps(r['header']['target'],sort_keys=True) for r in rows}),'graph_prefixes_checked':sum(r['graph_prefixes_checked'] for r in rows),'recovered_scalar_controls':sum(r['recovered'] is not None for r in rows),'observed_native_outputs_and_arithmetic_counts_equal':all(r['native_output_and_counts_equal'] for r in rows),'source_bindings_unchanged':True,'unexercised_events':[k for k in ['duplicate','identity','restart'] if not any(r['events'].get(k,0) for r in rows)],'independent_source_review':'pending','scientific_campaign_trials':0,'cost_ratio':None,'root_library_suite':'previous 8GiB allocation failure; 16GiB/600s compilation timeout retained in preceding packet'}
(p/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
s=(p/'REPORT.md').read_text()
s=re.sub(r'Final control receipt: [^;]+; existing native regression: [^.]+\.',f'Final control receipt: {q.name}; existing native regression: {n.name}.',s)
for a,b in [('seed23','seed 23'),('seed47','seed 47'),('node0','node 0'),('had30','had 30'),('the2048','the 2048'),('scalar2','scalar 2'),('after180','after 180'),('after179','after 179'),('full1,260','full 1,260')]:s=s.replace(a,b)
(p/'REPORT.md').write_text(s)
print(json.dumps(summary,indent=2))
