"""Assemble $SCRATCH/mini4 for the descent-walltime lane (tooling only; no science).

Real files (unchanged bytes) come from the frozen library commit fcda1e22
(extracted to /dev/shm/frozen by fetch_frozen.sh) plus the lane's nagao-e2e
overlay jv_cover.rs. Helper modules outside the closure are shims made of
byte-copied items. Tooling only; no science here.
"""
import os
import re
import shutil

import sys
SCR = sys.argv[1]
F = f"{SCR}/frozen/src"
OVERLAY = "/home/user/crypto/research/extension_ic_graph_20261115/nagao-e2e/library-overlay/jv_cover.rs"
LANE = "/home/user/crypto/research/extension_ic_graph_20261115/leads-20261118/complete-accounting/counted-linear-stage"
OUT = f"{SCR}/mini4"
C = f"{OUT}/src/cryptanalysis"
shutil.rmtree(OUT, ignore_errors=True)
os.makedirs(f"{C}/f4_fp")
os.makedirs(f"{OUT}/src/prime_hyperelliptic")

pat = re.compile(r'^(pub(\([a-z]+\))? )?(fn|struct|enum|type|const|static|impl(<[^>]*>)?|trait) +([A-Za-z0-9_]+)')


def extract(path, names):
    src = open(path).read().split("\n")
    n = len(src)

    def end(i):
        depth, started = 0, False
        for j in range(i, n):
            s = re.sub(r'//.*', '', src[j])
            s = re.sub(r'"(\\.|[^"\\])*"', '""', s)
            s = re.sub(r"'(\\.|[^'\\])'", "''", s)
            depth += s.count('{') - s.count('}')
            if '{' in s:
                started = True
            if started and depth <= 0:
                return j
            if not started and s.rstrip().endswith(';'):
                return j
        return n - 1
    out, i = [], 0
    while i < n:
        m = pat.match(src[i])
        if m:
            nm = m.group(5)
            if m.group(3).startswith('impl'):
                mm = re.match(r'^impl(<[^>]*>)? +(?:[A-Za-z0-9_:<>, ]+ for )?([A-Za-z0-9_]+)', src[i])
                nm = mm.group(2) if mm else nm
            e = end(i)
            if nm in names:
                k = i
                while k > 0 and (src[k - 1].startswith('///') or src[k - 1].startswith('#[')):
                    k -= 1
                out.append("\n".join(src[k:e + 1]))
            i = e + 1
        else:
            i += 1
    return "\n\n".join(out)


def copy(rel, dst=None):
    shutil.copyfile(f"{F}/{rel}", f"{OUT}/src/{dst or rel}")


for rel in ["cryptanalysis/f4_fp.rs", "cryptanalysis/f4_fp/reuse.rs", "cryptanalysis/f4_fp/pair_certificate.rs",
            "cryptanalysis/jv_bitops.rs", "cryptanalysis/jv_bitops_uniform.rs",
            "cryptanalysis/bitops.rs"]:
    copy(rel)
shutil.copyfile(OVERLAY, f"{C}/jv_cover.rs")
# Two linear stages: the frozen zero_dim_la.rs (byte-identical to Run50's
# 11c9a031, used for every timing run) and the complete-accounting lane's
# counted copy (pinned by SHA-256; selected only with --features counted).
import hashlib
orig = open(f"{F}/cryptanalysis/zero_dim_la.rs", "rb").read()
assert hashlib.sha256(orig).hexdigest() == "5ae85b3458c92dd4350604830966b1a83b240e2340f584e976ae2516ac4a841d"
counted = open(f"{LANE}/zero_dim_la.rs", "rb").read()
assert hashlib.sha256(counted).hexdigest() == "f0c50f5f3cf8c9150932524b45255a2ee0a948f76146c2b3775aa16d33e17ec5", "counted zero_dim_la changed"
open(f"{C}/zero_dim_la.rs", "wb").write(orig)
open(f"{C}/zero_dim_la_counted.rs", "wb").write(counted)

# prime_hyperelliptic::ops minus divisor_hash_words (needs the curve module; unused here)
ops = open(f"{F}/prime_hyperelliptic/ops.rs").read()
a = ops.index("/// Words a hash of a divisor absorbs")
b = ops.index("/// Everything this thread has tallied, by phase.")
ops2 = ops[:a] + ops[b:]
# lib-test builds: drop ops.rs's own test module (it needs the curve types)
t = ops2.index("#[cfg(test)]")
open(f"{OUT}/src/prime_hyperelliptic/ops.rs", "w").write(ops2[:t])
# bitops.rs tests include the unit vectors by relative path
os.makedirs(f"{OUT}/docs/units", exist_ok=True)
for v in ["bitop-v1.vectors.json", "bitop-v2.vectors.json"]:
    shutil.copyfile(f"/home/user/crypto/docs/units/{v}", f"{OUT}/docs/units/{v}")
open(f"{OUT}/src/prime_hyperelliptic/mod.rs", "w").write("//! Shim: only ops.rs (divisor_hash_words removed).\npub mod ops;\n")

fx = open(f"{F}/cryptanalysis/fx_hash.rs").read().split("\n")
keep = [l for l in fx[:fx.index("/// A hasher for maps keyed by one monomial mask.")] if "pq_groebner_f2::Mask" not in l]
open(f"{C}/fx_hash.rs", "w").write("//! Shim: fx_hash.rs up to the Mask hashers (byte-copied).\n" + "\n".join(keep) + "\n")

gc_head = open(f"{F}/cryptanalysis/gaudry_cubic.rs").read().split("\n")
uses = [l for l in gc_head[:120] if l.startswith("use ") and "residual_walk" not in l]
open(f"{C}/gaudry_cubic.rs", "w").write(
    "//! Shim: byte-copied items of gaudry_cubic.rs (frozen fcda1e22).\n#![allow(dead_code, unused_imports)]\n"
    + "\n".join(uses) + "\nuse super::residual_walk::{inv_mod, is_prime_u64, mix64, pow_mod};\n"
    + extract(f"{F}/cryptanalysis/gaudry_cubic.rs",
              {"mm", "am", "sm", "UPoly", "PolyRing", "rref_mod_p", "SparseRel", "square_core", "wiedemann_u64", "SquareCore", "trim", "berlekamp_massey_u64", "equal_solver_words"}) + "\n")
open(f"{C}/residual_walk.rs", "w").write(
    "//! Shim: byte-copied items of residual_walk.rs (frozen fcda1e22).\n"
    + extract(f"{F}/cryptanalysis/residual_walk.rs", {"pow_mod", "inv_mod", "is_prime_u64", "mix64", "sqrt_mod", "mul_mod"}) + "\n")
open(f"{C}/glv_invariant_base.rs", "w").write(
    "//! Shim: byte-copied items of glv_invariant_base.rs (frozen fcda1e22).\n#![allow(unused_imports)]\n"
    + "use crate::cryptanalysis::residual_walk::{inv_mod, is_prime_u64, pow_mod, sqrt_mod};\n"
    + "use rand::rngs::StdRng;\nuse rand::{Rng, SeedableRng};\nuse std::collections::VecDeque;\n"
    + extract(f"{F}/cryptanalysis/glv_invariant_base.rs", {"poly_roots_fp_counted_until", "poly_gcd", "poly_trim", "mulm", "negm", "poly_powmod", "subm", "addm", "poly_mulmod", "poly_rem", "poly_divrem", "poly_mul", "poly_sub", "poly_add", "poly_monic", "poly_eval", "poly_deg"}) + "\n")
open(f"{C}/groebner_f4.rs", "w").write(
    "//! Shim: byte-copied items of groebner_f4.rs (frozen fcda1e22).\n"
    + extract(f"{F}/cryptanalysis/groebner_f4.rs", {"Ordering", "cmp_monomial"}) + "\n")
open(f"{C}/ic_measurement.rs", "w").write(
    "//! Shim for ic_measurement::callgrind_dump: the real call is a Valgrind client\n"
    "//! request, inert outside Valgrind; this local control never runs under it.\n"
    "pub(crate) fn callgrind_dump(_label: &'static [u8]) {}\n")
open(f"{C}/mod.rs", "w").write(
    "#![allow(dead_code, unused_imports, unexpected_cfgs, clippy::all)]\n"
    + "#[cfg(not(feature = \"counted\"))]\npub mod zero_dim_la;\n"
    + "#[cfg(feature = \"counted\")]\n#[path = \"zero_dim_la_counted.rs\"]\npub mod zero_dim_la;\n"
    + "".join(f"pub mod {m};\n" for m in ["bitops", "jv_bitops", "f4_fp", "jv_cover", "fx_hash",
                                          "gaudry_cubic", "glv_invariant_base", "groebner_f4", "ic_measurement", "residual_walk"]))
open(f"{OUT}/src/lib.rs", "w").write(
    "//! Minimal local build of the frozen Nagao descent closure (descent-finder lane).\n"
    "#![allow(dead_code, unused_imports, unexpected_cfgs, clippy::all)]\npub mod cryptanalysis;\npub mod prime_hyperelliptic;\n")
# Cargo.toml is written by build.sh from Cargo.toml.in (the original session
# copied /dev/shm/mini/Cargo.toml, identical apart from the bin path).

print("assembled", OUT)
