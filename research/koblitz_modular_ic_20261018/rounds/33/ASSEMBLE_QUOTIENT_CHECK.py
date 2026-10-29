#!/usr/bin/env python3
"""Reuse exact frozen point drivers, no field arithmetic."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent;p=h/'runs/quotient33-function01';s=(p/'native.log').read_text()
assert (p/'exit.txt').read_text().strip()=='0' and 'QUOTIENT33_FUNCTION_ALL_CHECKS_PASS' in s and not re.search(r'^\s*\?',s,re.M)
func=(h/'source-data/quotient-family-function.sing').read_text();oldbody=(h/'LOCAL33-CHECK.sing').read_text();body=(h/'QUOTIENT33-CHECK.sing').read_text()
for degree in (5,83):
 old=(h/f'source-data/local-check{degree}-input-template.sing').read_text();start=old.index('ring pairedImage3=') if degree==83 else old.index('ring localField5=')
 driver=old[start:];assert oldbody in driver
 s=func+'\n'+driver.replace(oldbody,body).replace('checkLocal33(','checkQuotient33(').replace('casesLocal33','casesQC33').replace(f'LOCAL33_CHECK{degree}_ALL_CHECKS_PASS',f'QUOTIENT33_CHECK{degree}_ALL_CHECKS_PASS')
 (h/f'source-data/quotient-check{degree}-input-template.sing').write_text(s)
print('Assembled all two padding and ten degree83 local-presentation point checks.')
