#!/usr/bin/env python3
"""Reuse identical point-driver text with the natively roundtripped family."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent;p=h/'runs/local33-function01'
s=(p/'native.log').read_text();assert (p/'exit.txt').read_text().strip()=='0' and 'LOCAL33_FUNCTION_ALL_CHECKS_PASS' in s and not re.search(r'^\s*\?',s,re.M)
func=(h/'source-data/local-family-function.sing').read_text();body=(h/'LOCAL33-COMPACT-CHECK.sing').read_text()
oldbody=(h/'LOCAL33-CHECK.sing').read_text()
for degree in (83,5):
 old=(h/f'source-data/local-check{degree}-input-template.sing').read_text()
 # Keep the exact field/point driver from the earlier frozen input.
 start=old.index('ring pairedImage3=') if degree==83 else old.index('ring localField5=')
 field_and_driver=old[start:];assert oldbody in field_and_driver
 out=func+'\n'+field_and_driver.replace(oldbody,body).replace('checkLocal33(', 'checkCompact33(').replace('casesLocal33','casesCompact33')
 out=out.replace(f'LOCAL33_CHECK{degree}_ALL_CHECKS_PASS',f'LOCAL33_COMPACT_CHECK{degree}_ALL_CHECKS_PASS')
 (h/f'source-data/local-compact-check{degree}-input-template.sing').write_text(out)
print('Assembled equivalent compact checks with identical point data and51 witnesses.')
