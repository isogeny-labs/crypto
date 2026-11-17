"""Run public McGuire--Mueller definitions with the local validation harness.

This optional check needs network access. It sends public mathematical code to
the University of Sydney's public Magma calculator, discovered from its page
https://magma-maths.org/calc/. This is a small correctness check, not a benchmark.
"""
import hashlib
import json
from pathlib import Path
from urllib.request import Request, urlopen

root = Path(__file__).resolve().parent
source_url = 'https://math.ie/ecdlp/sourcecode.magma'
local_source = root / 'sources' / 'mcguire-code.magma'
if local_source.exists():
    source_bytes = local_source.read_bytes()
else:
    with urlopen(source_url,timeout=25) as response:
        source_bytes = response.read()
source = source_bytes.decode('utf-8')
assert '/*EXAMPLE*/' in source
definitions = source.split('/*EXAMPLE*/')[0]
code = definitions + '\n' + (root / 'magma_check_harness.magma').read_text()
request = Request('https://calculator.magma-maths.org/execute',
                  data=json.dumps({'code':code}).encode(),
                  headers={'Content-Type':'application/json',
                           'Origin':'https://magma-maths.org'})
with urlopen(request,timeout=55) as response:
    result = json.load(response)
result['source_url'] = source_url
result['source_sha256'] = hashlib.sha256(source_bytes).hexdigest()
result['scope'] = 'Unchanged author definitions; fixed small curve; own driver.'
(root / 'magma-author-code-check.json').write_text(json.dumps(result,indent=2)+'\n')
print(result.get('stdout',result))
print('Runtime metadata:',result.get('magma'))
assert result.get('success') is True,result
assert 'ALL SIX CHECKS PASSED' in result.get('stdout',''),result
