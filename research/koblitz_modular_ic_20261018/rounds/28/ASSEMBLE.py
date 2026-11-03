#!/usr/bin/env python3
"""Copy pinned native helpers, preserving all mathematical work in Rust."""
from pathlib import Path
import hashlib
here=Path(__file__).resolve().parent
prior=here.parents[1]/'native'
# here=study/rounds/28; parents[1]=study
ordinary=(prior/'src/ordinary13.rs').read_text().split('pub fn run(dir: &Path)')[0]
ordinary=ordinary.replace('//!','//')
ordinary=ordinary.replace('use crate::{round02, round04::Field};\n','').replace('#[path = "ordinary13_factor.rs"]\npub(crate) mod factor;\n','')
field=(prior/'src/round04.rs').read_text();field=field[field.index('#[derive(Clone, Copy, Debug)]'):field.index('pub type Poly')]
main='#![allow(dead_code)]\n'+field+'\n'+ordinary+'\n'+(here/'DUAL28.rs').read_text()
(here/'native/src/main.rs').write_text(main)
(here/'native/Cargo.toml').write_bytes((prior/'Cargo.toml').read_bytes())
(here/'native/Cargo.lock').write_bytes((prior/'Cargo.lock').read_bytes())
(here/'native/source.sha256').write_text(hashlib.sha256(main.encode()).hexdigest()+'  src/main.rs\n')
