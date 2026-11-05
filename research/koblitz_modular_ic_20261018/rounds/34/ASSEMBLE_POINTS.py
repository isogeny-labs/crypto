#!/usr/bin/env python3
"""Reuse exact frozen point drivers, no field arithmetic."""
from pathlib import Path
import re,hashlib
h=Path(__file__).resolve().parent;p=h/'runs/prefix34-function01';s=(p/'native.log').read_text()
assert (p/'exit.txt').read_text().strip()=='0' and 'PREFIX34_FUNCTION_ALL_CHECKS_PASS' in s and not re.search(r'^\s*\?',s,re.M)
donor_manifest={name:d for d,name in (line.split(maxsplit=1) for line in (h.parent/'33/runs.sha256').read_text().splitlines())}
for degree in (5,83):
 name=f'source-data/local-check{degree}-input-template.sing';assert hashlib.sha256((h.parent/'33'/name).read_bytes()).hexdigest()==donor_manifest[name]
func=(h/'source-data/quotient-family-function.sing').read_text();oldbody=(h.parent/'33/LOCAL33-CHECK.sing').read_text();body=(h/'PREFIX34-CHECK.sing').read_text()
for degree in (5,83):
 old=(h.parent/f'33/source-data/local-check{degree}-input-template.sing').read_text();start=old.index('ring pairedImage3=') if degree==83 else old.index('ring localField5=')
 driver=old[start:];assert oldbody in driver
 s=func+'\n'+driver.replace(oldbody,body).replace('checkLocal33(','checkQuotient33(').replace('casesLocal33','casesQC33').replace(f'LOCAL33_CHECK{degree}_ALL_CHECKS_PASS',f'PREFIX34_CHECK{degree}_ALL_CHECKS_PASS')
 (h/f'source-data/quotient-check{degree}-input-template.sing').write_text(s)
print('Assembled all two padding and ten degree83 local-presentation point checks.')
