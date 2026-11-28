from pathlib import Path
import os,sys,json,hashlib,tarfile,io,base64,lzma
C=Path(__file__).resolve().parent;R=C.parents[1];os.chdir(R);sys.path.insert(0,str(R));saved={};encoded=''.join((C/name).read_text().strip()for name in json.loads((C/'RAW_ARCHIVE_PARTS.json').read_bytes()))
with tarfile.open(fileobj=io.BytesIO(base64.b64decode(encoded)),mode='r:xz')as t:
 for name,h in json.loads((C/'RAW_INDEX.json').read_bytes()).items():
  b=t.extractfile(name).read();assert hashlib.sha256(b).hexdigest()==h,name;saved[name]=b
for name,h in json.loads((C/'SOURCE_SHA256.json').read_bytes()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==h,name
from research.unit_cofactor_search_20261117_v135b.recovery import oracles
from research.unit_cofactor_20261117_v134b.kernel import transform
from research.correction_basis_checks_20261117_v128b.kernel import compile_all
P=R/'research/unit_cofactor_search_20261117_v135b';data=json.loads((P/'DATA.json').read_bytes());raw=lzma.decompress(saved['research/unit_cofactor_search_20261117_v135b/INSTANCE.json.xz']);assert hashlib.sha256(raw).hexdigest()==data['instance_sha256'];assert oracles()==12
p=json.loads(saved['research/unit_cofactor_search_20261117_v135b/search_raw.json']);assert p['out']['recovery_oracle_checks']==12;cfg=json.loads((R/'research/chart_compiler_20261116_v112/DATA.json').read_bytes());chart=json.loads((R/'research/leading_cover_20261116_v116/cover.json').read_bytes())['records'][1]['chart'];G,oo,rr,meta=compile_all(cfg,chart,8);H,nn,roots,selectors,h,stats=transform(G,oo,rr,oo[-1]['wires'][0][0],int(data['direction_hex'],16),1);b=json.dumps({'nodes':H.nodes,'outputs':nn,'roots':roots,'selector_readback':selectors,'cofactor':h},separators=(',',':')).encode();assert hashlib.sha256(b).hexdigest()==p['out']['graph_sha256']
print('C135 immutable source/raw hashes,original-instance snapshot,12independent recovery oracles and changed graph replayed;native search not rerun;noqualifiedspeedratio.')
