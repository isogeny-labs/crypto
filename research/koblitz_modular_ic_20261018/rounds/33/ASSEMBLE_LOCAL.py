#!/usr/bin/env python3
"""Source concatenation only."""
from pathlib import Path
h=Path(__file__).resolve().parent
s=''.join((h.parent/'27/LAMBDA14.sing').read_text().splitlines(True)[:254])+'\n'+(h/'LOCAL33-GENERATE.sing').read_text()
(h/'source-data/local-generate-input-template.sing').write_text(s)
print('Assembled original source numerators and exact local-presentation producer.')
