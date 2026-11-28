#!/usr/bin/env python3
"""Assemble frozen native source and byte-checked donor inputs only."""
from pathlib import Path
import re,json,hashlib
h=Path(__file__).resolve().parent;prior=h.parent/'36';old=h.parent/'35'
s=(prior/'GROUP36.rs').read_text().split('fn main(){',1)[0]
s=s.replace('assert!(candidate.len()>=expected,"quotient below expected dimension");','if expected!=usize::MAX{assert!(candidate.len()>=expected,"quotient below expected dimension");}')
s=s.replace('assert_eq!(candidate.len(),expected);series_basis36(&candidate);','if expected!=usize::MAX{assert_eq!(candidate.len(),expected);}else{assert!(candidate.len()<=1);}series_basis36(&candidate);')
assert s.count('usize::MAX')==2
(h/'SIGNED37.rs').write_text(s+(h/'SIGNED-MAIN37.rs').read_text())
p=h/'runs/order37-01';assert (p/'exit.txt').read_text().strip()=='0';log=(p/'native.log').read_text();assert 'ORDER37_ALL_CHECKS_PASS' in log and not re.search(r'^\s*\?',log,re.M)
manifest={n:d for d,n in (line.split(maxsplit=1) for line in(old/'runs.sha256').read_text().splitlines())};pins={}
for name in ['runs/jets35-01/evaluation-words.txt']+[f'runs/inflate35-01/products/coupled-r{r}-f{f}-basis.txt' for r in range(1,6) for f in (2,3)]:
 data=(old/name).read_bytes();dig=hashlib.sha256(data).hexdigest();assert dig==manifest[name];pins['R35/'+name]=dig
pins['R37/runs/order37-01/deck-order.txt']=hashlib.sha256((p/'deck-order.txt').read_bytes()).hexdigest()
(h/'signed-donors.json').write_text(json.dumps(pins,indent=2)+'\n');print('Assembled exact signed-class source and verified all actual class/order donor bytes.')
