#!/usr/bin/env python3
"""Accepted input/receipt byte binding and literal native source assembly."""
from pathlib import Path
import hashlib,json,re
h=Path(__file__).resolve().parent;p=h/'runs/cofactor37-01';assert (p/'exit.txt').read_text().strip()=='0';log=(p/'native.log').read_text();assert 'COFACTOR37_ALL_CHECKS_PASS' in log and not re.search(r'^\s*\?',log,re.M);assert 'minus=3721' in log
s=(h/'TORSION37.rs').read_text().split('fn main(){',1)[0]+(h/'ISOLATE-MAIN37.rs').read_text();(h/'ISOLATE37.rs').write_text(s)
names=['runs/cofactor37-01/complementary-euler.sing','runs/cofactor37-01/native.log','runs/signed37-01/classes/signed-first-basis.txt','runs/signed37-01/classes/signed-second-basis.txt'];(h/'isolate-donors.json').write_text(json.dumps({n:hashlib.sha256((h/n).read_bytes()).hexdigest() for n in names},indent=2)+'\n');print('Bound accepted cofactor and two nonprincipal signed-class representatives.')
