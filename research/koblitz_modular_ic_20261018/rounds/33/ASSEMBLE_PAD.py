#!/usr/bin/env python3
"""Literal source assembly; no mathematical computation."""
from pathlib import Path
h=Path(__file__).resolve().parent
source=''.join((h.parent/'27/LAMBDA14.sing').read_text().splitlines(True)[:254])
old=(h.parent/'31/REVERSE31.sing').read_text();law=old[old.index('proc add31('):old.index('proc source31(')]
(h/'source-data/pad-input-template.sing').write_text(source+'\n'+law+'\n'+(h/'PAD33.sing').read_text())
print('Assembled exhaustive ordinary F32 padding-point input with original residuals and independent Tate law.')
