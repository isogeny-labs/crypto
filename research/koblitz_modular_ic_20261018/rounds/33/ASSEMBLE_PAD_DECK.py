#!/usr/bin/env python3
"""Literal accepted padding representatives and original equations."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent;p=h/'runs/pad33-01';s=(p/'native.log').read_text()
assert (p/'exit.txt').read_text().strip()=='0' and 'PAD33_ALL_CHECKS_PASS' in s and not re.search(r'^\s*\?',s,re.M)
s=''.join((h.parent/'27/LAMBDA14.sing').read_text().splitlines(True)[:254])
s+='\nring paddingDeckField33=(2,a),(T,w),dp;minpoly=a^5+a^2+1;\n'+(p/'padding-representatives.sing').read_text()+'\n'+(h/'PAD33-DECK.sing').read_text()
(h/'source-data/padding-deck-input-template.sing').write_text(s)
print('Assembled actual closed padding-point diamond and Frobenius controls.')
