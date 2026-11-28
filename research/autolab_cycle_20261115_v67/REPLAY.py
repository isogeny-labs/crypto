from pathlib import Path
import json,hashlib
R=Path(__file__).resolve().parents[2];P=Path(__file__).resolve().parent
for n,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
for d in ['image_cells_20261115_v67','image_cells_actual_20261115_v67','image_cell_certificates_20261115_v67']:assert json.loads((R/'research'/d/'raw.json').read_text())['error']is None
s=json.loads((R/'research/image_cell_certificates_20261115_v67/raw.json').read_text())['out']
for r in s['cells']:
 if r['dual_certificate_hex']:
  dual=int(r['dual_certificate_hex'],16);cs=[int(x,16)for x in r['coefficient_hex']];assert(cs[0]&dual).bit_count()%2==1;assert all((x&dual).bit_count()%2==0 for x in cs[1:])
assert s['cells'][-1]['dual_certificate_hex']is None
print('Hashes and exact whole-cell dual certificates verified;construction is enumerative andqualifiedspeedupnull.')
