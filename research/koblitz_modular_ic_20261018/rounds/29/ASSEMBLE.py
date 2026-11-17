#!/usr/bin/env python3
from pathlib import Path
import hashlib
here=Path(__file__).resolve().parent
prior=here/'../28/native'
source=(prior/'src/main.rs').read_text()
assert source.count('fn main(){')==1
source=source.replace('fn main(){','fn previous28_main(){')+'\n'+(here/'JET29.rs').read_text()
(here/'native/src/main.rs').write_text(source)
for name in ['Cargo.toml','Cargo.lock']:(here/'native'/name).write_bytes((prior/name).read_bytes())
(here/'native/source.sha256').write_text(hashlib.sha256(source.encode()).hexdigest()+'  src/main.rs\n')
