#!/usr/bin/env python3
"""Literal accepted multiplication table assembly."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent;p=h/'runs/desc33-02';s=(p/'native.log').read_text()
assert (p/'exit.txt').read_text().strip()=='0' and 'DESC33_ALL_CHECKS_PASS' in s and not re.search(r'^\s*\?',s,re.M)
lines=(p/'products2.sing').read_text().splitlines()
selected=[s for s in lines if s.startswith(('sectionPairs2[','sectionCoordinates2['))]
assert len(selected)==2542
s='ring quadCoordinates33=2,(w),dp;intmat sectionPairs2[196][2];ideal sectionCoordinates2;\n'+'\n'.join(selected)+'\n'+(h/'QUADRICS33.sing').read_text()
(h/'source-data/quadrics-input-template.sing').write_text(s)
print('Assembled all2346 accepted product coordinates for explicit twist quadrics.')
