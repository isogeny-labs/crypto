#!/usr/bin/env python3
"""Build a standalone crate around the lab's byte-identical f4_fp.rs.

Why: at base 71a7881ede the crypto lib does not compile
(src/cryptanalysis/prime_root_trace.rs:1108, unclosed `{`), and this task may
not edit existing files. f4_fp.rs (and f4_fp/*.rs) are included UNCHANGED via
#[path]. Helpers it calls are copied verbatim by line range (hashes recorded);
jv_bitops (op-count instrumentation only) is replaced by no-op stubs.
Usage: gen_shim.py <outdir>
"""
import hashlib, json, sys
from pathlib import Path
REPO = Path(__file__).resolve().parents[3]
SRC = REPO / 'src/cryptanalysis'
out = Path(sys.argv[1]); (out / 'src').mkdir(parents=True, exist_ok=True)
def lines(f, a, b):
    return ''.join((SRC / f).read_text().splitlines(True)[a-1:b])
glv = lines('glv_invariant_base.rs', 96, 107) + lines('glv_invariant_base.rs', 1073, 1250)
rw = lines('residual_walk.rs', 112, 124) + lines('residual_walk.rs', 192, 237) + lines('residual_walk.rs', 290, 320)
gf4 = lines('groebner_f4.rs', 66, 104)
fx = (SRC / 'fx_hash.rs').read_text()
cr = f'''    pub mod f4_fp;
    pub mod fx_hash;
    pub mod pq_groebner_f2 {{ pub type Mask = u64; }}
    pub mod groebner_f4 {{
{gf4}
    }}
    pub mod glv_invariant_base {{
        use rand::rngs::StdRng;
        use rand::{{Rng, SeedableRng}};
        use super::residual_walk::inv_mod;
{glv}
    }}
    pub mod residual_walk {{
{rw}
    }}
    /// No-op replacement of the op-count instrumentation module.
    pub mod jv_bitops {{
        #[derive(Clone, Copy, Debug)] pub enum Kind {{ FieldAdd, FieldMul, FieldInv }}
        pub fn enabled() -> bool {{ false }}
        pub fn charge(_k: Kind, _n: u64) {{}}
        pub fn row_ops(_n: u64) {{}}
        pub fn words(_n: u64) {{}}
        pub fn probes(_n: u64, _k: u64) {{}}
        pub fn hook_inv(_m: u64) {{}}
        pub fn hook_pow(_m: u64, _e: u64) {{}}
        pub fn sort_cost(_k: usize) -> u64 {{ 0 }}
    }}
'''
lib = "#![allow(dead_code, unused_imports, unused_variables, clippy::all)]\npub mod cryptanalysis;\n"
(out / 'src/cryptanalysis.rs').write_text(cr)
d = out / 'src/cryptanalysis'; d.mkdir(exist_ok=True)
for name in ('f4_fp.rs', 'f4_fp', 'fx_hash.rs'):
    link = d / name
    if not link.exists():
        link.symlink_to(SRC / name)
(out / 'src/lib.rs').write_text(lib)
cargo = f'''[package]
name = "crypto_lib"
version = "0.0.0"
edition = "2021"
[workspace]
[lib]
path = "src/lib.rs"
[[bin]]
name = "f4arm"
path = "{REPO}/examples/rootfind_matched_f4_gate_20261123.rs"
[dependencies]
rayon = "1"
rand = "0.8"
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
certified-relations = {{ path = "{REPO}/crates/certified-relations" }}
relation-replay-checker = {{ path = "{REPO}/crates/relation-replay-checker" }}
[profile.release]
codegen-units = 4
'''
(out / 'Cargo.toml').write_text(cargo)
hashes = {str(p.relative_to(REPO)): hashlib.sha256(p.read_bytes()).hexdigest()
          for p in [SRC/'f4_fp.rs', *sorted((SRC/'f4_fp').glob('*.rs')), SRC/'fx_hash.rs',
                    SRC/'glv_invariant_base.rs', SRC/'residual_walk.rs', SRC/'groebner_f4.rs',
                    REPO/'examples/rootfind_matched_f4_gate_20261123.rs']}
(out / 'SOURCE_HASHES.json').write_text(json.dumps(hashes, indent=1))
print(json.dumps(hashes, indent=1))
