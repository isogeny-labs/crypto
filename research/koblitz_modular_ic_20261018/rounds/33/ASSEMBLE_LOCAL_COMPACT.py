#!/usr/bin/env python3
"""Concatenate identical native generic prefix and rational serializer."""
from pathlib import Path
h=Path(__file__).resolve().parent
s=''.join((h.parent/'27/LAMBDA14.sing').read_text().splitlines(True)[:254])+'\n'
s+=(h/'LOCAL33-GENERATE.sing').read_text().split('proc emitLoc33(')[0]
s+='\n'+(h/'LOCAL33-COMPACT-DUMP.sing').read_text()
(h/'source-data/local-compact-dump-input-template.sing').write_text(s)
print('Assembled identical generic family with compact rational serialization and native roundtrip.')
