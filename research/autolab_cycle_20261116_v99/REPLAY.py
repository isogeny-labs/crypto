from pathlib import Path
import json,hashlib,sys,io,tarfile,lzma,base64
C=Path(__file__).resolve().parent;ROOT=C.parents[1];sys.path.insert(0,str(ROOT));index=json.loads((C/'RAW_INDEX.json').read_text());members={}
with tarfile.open(fileobj=io.BytesIO(lzma.decompress(base64.b64decode((C/'RAW_EVIDENCE.tar.xz.b64').read_text()))))as tf:
 assert set(tf.getnames())==set(index)
 for name,h in index.items():
  raw=tf.extractfile(name).read();assert hashlib.sha256(raw).hexdigest()==h;members[name]=raw
for name,h in json.loads((C/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==h,name
def data(name):return json.loads(members[name])
P=ROOT/'research/residue_quotient_20261116_v99b'
for r in json.loads((P/'RECOVERY.json').read_text()):
 raw=(ROOT/r['local_path']).read_bytes();assert hashlib.sha1(b'blob '+str(len(raw)).encode()+b'\0'+raw).hexdigest()==r['git_blob_sha']
a=data('research/residue_quotient_20261116_v99/oracles_raw.json');assert a['error']is not None and not a['out']['toy']
a=data('research/residue_quotient_20261116_v99b/oracles_raw.json');assert a['error']is None and len(a['out']['toy'])==80 and len(a['out']['recovered'])==12
from research.residue_quotient_20261116_v99b.problem import build
from research.residue_quotient_20261116_v99b.kernel import count,basis,poly
E,eb,widths,tables,F,fb,last,details=build(P);a=count(E,eb,widths,tables,F,fb,last,'dp');b=count(E,eb,widths,tables,F,fb,last,'walsh');assert a['distribution']==b['distribution'];assert a['exact_necessary_parameter_count']==b['exact_necessary_parameter_count']==274316578275 and a['quotient_bits']==12 and a['kernel_bits']==a['last_kernel_rank']==4
assert a['first_table_product']*16==18552757630176 and a['original_complete']is False and a['original_exact_root_count']is None
c=data('research/residue_quotient_20261116_v99b/profile_raw.json');assert c['error']is None and c['out']['methods']['dp']['result']['distribution']==a['distribution']
old=json.loads((P/'snapshot/research__residue_lift_sieve_20261116_v84__raw.json').read_text())['out']['tables']
from itertools import combinations_with_replacement
for row in old:
 s,m=row['degree'],row['modulus'];allowed={sum(v<<(s*i)for i,v in enumerate(poly(xs,m)[:-1]))for xs in combinations_with_replacement(range(1<<s),5)};assert len(allowed)==row['allowed_count']and hashlib.sha256(json.dumps(sorted(allowed)).encode()).hexdigest()==row['allowed_sha256']
assert json.loads((C/'LEDGER_VALIDATION.txt').read_text())['integrity']=='passed';print('C99 exactgitbytes,retained failedsnapshot,80toychecks,all8localtablehashes,and exact4096-state necessaryparametercount verified. Originalrootcompletion remainsfalse.')
