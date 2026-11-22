from pathlib import Path
import json,hashlib
P=Path(__file__).resolve().parent;R=P.parents[1]
for name,digest in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/name).read_bytes()).hexdigest()==digest
D=R/'research/root_validated_cold_20261115_v50';counts=[]
for index in range(4):
 a=json.loads((D/('preflight_reference_h'+str(index)+'.json')).read_text());b=json.loads((D/('preflight_compressed_h'+str(index)+'.json')).read_text());key=lambda x:{(v,int(t,16))for v,t in x['native']['candidates']};assert a['norm_family_complete']and b['norm_family_complete']and key(a)==key(b)and a['supported']==b['supported']==[];counts.append(len(key(a)))
assert counts==[249,244,266,257];print('Verified complete four-chart candidate/norm receipts; no repeated timing claim.')
