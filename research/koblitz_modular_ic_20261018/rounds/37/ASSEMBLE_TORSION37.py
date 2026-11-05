#!/usr/bin/env python3
"""Literal native-source assembly and accepted receipt/donor-byte binding."""
from pathlib import Path
import hashlib,re,json
h=Path(__file__).resolve().parent;p=h/'runs/signed37-01'
assert (p/'exit.txt').read_text().strip()=='0';log=(p/'native.log').read_text();assert 'SIGNED37_ALL_CHECKS_PASS' in log and 'signed_class_principal=false' in log
assert (p/'classes/signed-result.txt').read_text().startswith('principal=false\n')
s=(h/'SIGNED37.rs').read_text().split('fn main(){',1)[0]+(h/'TORSION-MAIN37.rs').read_text();(h/'TORSION37.rs').write_text(s)
names=['classes/signed-first-basis.txt','classes/signed-second-basis.txt','classes/signed-result.txt','native.log','exit.txt'];pins={n:hashlib.sha256((p/n).read_bytes()).hexdigest() for n in names};(h/'torsion-donors.json').write_text(json.dumps(pins,indent=2)+'\n');print('Bound two accepted nonprincipal representatives and froze native scalar-test source.')
