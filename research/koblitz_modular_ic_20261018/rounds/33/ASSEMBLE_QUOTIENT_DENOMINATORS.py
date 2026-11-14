#!/usr/bin/env python3
"""Literal accepted native export assembly, without arithmetic."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent;p=h/'runs/local33-quotient01'
s=(p/'native.log').read_text()
assert (p/'exit.txt').read_text().strip()=='0' and 'LOCAL33_QUOTIENT_ALL_CHECKS_PASS' in s and not re.search(r'^\s*\?',s,re.M)
small=(h/'runs/local33-compact-dump01/rational-family.sing').read_text().splitlines()
s=(p/'direct-quotients.sing').read_text()+'\nideal rationalSource33;\n'
s+='\n'.join(x for x in small if x.startswith('rationalSource33['))+'\n'+(h/'QUOTIENT33-DENOMINATORS.sing').read_text()
(h/'source-data/quotient-denominators-input-template.sing').write_text(s)
print('Assembled all four completed quotient families, exact lifts and coefficient-domain checks.')
