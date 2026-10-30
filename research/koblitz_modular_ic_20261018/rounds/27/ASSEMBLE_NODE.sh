#!/usr/bin/env bash
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
cd "$here"
sha256sum -c node-inputs.sha256
mkdir -p source-data
python3 - <<'PY'
import json
from pathlib import Path
records=json.loads(Path('../22/runs/q41-node187-coupled-valuations.json').read_text())['records']
with Path('source-data/node-input-template.sing').open('w')as out:
 out.write(Path('NODE-header.sing').read_text())
 for graph,r in enumerate(records):
  assert r['graph']==graph and len(r['functions'])==4
  for family,fn in enumerate(r['functions']):
   assert len(fn['five_conjugates'])==5
   for k,c in enumerate(fn['five_conjugates']):
    assert c['coefficient_sigma_power']==k and c['D']==['1']
    index=graph*20+family*5+k+1
    for field,name in [('P','millerP'),('S','millerS')]:
     terms=['hexField("'+value+'")*x^'+str(i) for i,value in enumerate(c[field])]
     out.write(name+'['+str(index)+']='+'+'.join(terms)+';\n')
 out.write(Path('NODE27.sing').read_text())
PY
sha256sum source-data/node-input-template.sing > source-data/node-input.sha256
tar -cJf checkpoint-live.tar.xz runs source-data
