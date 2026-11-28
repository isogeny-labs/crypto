from pathlib import Path
import sys,json,hashlib
R=Path(__file__).resolve().parents[2];P=Path(__file__).resolve().parent;sys.path.insert(0,str(R))
from research.quadratic_cell_separator_20261115_v68.separator import compose_or,zeta
for n,h in json.loads((P/'EVIDENCE_SHA256.json').read_text()).items():assert hashlib.sha256((R/n).read_bytes()).hexdigest()==h,n
old=json.loads((R/'research/image_cell_certificates_20261115_v67/raw.json').read_text())['out']['cells'];first=json.loads((R/'research/quadratic_cell_separator_20261115_v68/raw.json').read_text());assert first['error']is None
for source,record in zip(old,first['out']['actual']):assert compose_or([int(x,16)for x in source['coefficient_hex']],[int(x,16)for x in record['forms_hex']])==1
fresh=json.loads((R/'research/fresh_cell_separators_20261115_v68/raw.json').read_text());assert fresh['error']is None
for row in fresh['out']['cells']:
 if row['forms_hex']is not None:assert compose_or([int(x,16)for x in row['coefficient_hex']],[int(x,16)for x in row['forms_hex']])==1
 else:assert 0 in zeta([int(x,16)for x in row['coefficient_hex']])
print('Exact nonlinear cell certificates and supported-root control verified;no root-search speedup claim.')
