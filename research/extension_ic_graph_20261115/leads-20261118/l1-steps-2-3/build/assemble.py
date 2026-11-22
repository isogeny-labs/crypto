"""Assemble $SCRATCH/mini23 for the l1-steps-2-3 lane. Tooling only; no science.

Library = the Root41-equivalent fcda1e22 closure (real files, unchanged bytes) for
jv_sieve (+graph, two_large_prime, its dlp/counted_linear/homogeneous_quotient/
extension_remainder_dlp/principal_relation submodules), large_prime_filter(+mst),
f4_fp, zero_dim_la, jv_bitops(+uniform), bitops; jv_cover is the nagao-e2e
overlay (fcda1e22 jv_cover plus the optional nagao_decompose_using callback; with
no callback the code path is the original). Helper modules outside the closure
are shims of byte-copied items, as in ../../l1-pattern-tally/build/assemble.py.
The 11c9a031 deferred-pair modules stay in $SCRATCH/run50src and are compiled
into the driver binary by #[path], as Run50's example did.
"""
import hashlib, os, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from extract_helper import extract

SCR = sys.argv[1]
F = f"{SCR}/frozen/src"
OVERLAY = "/home/user/crypto/research/extension_ic_graph_20261115/nagao-e2e/library-overlay/jv_cover.rs"
OUT = f"{SCR}/mini23"
C = f"{OUT}/src/cryptanalysis"
shutil.rmtree(OUT, ignore_errors=True)
os.makedirs(f"{C}/f4_fp")
os.makedirs(f"{OUT}/src/prime_hyperelliptic")


def copy(rel):
    shutil.copyfile(f"{F}/{rel}", f"{OUT}/src/{rel}")


for rel in ["f4_fp.rs", "f4_fp/reuse.rs", "f4_fp/pair_certificate.rs", "jv_bitops.rs", "jv_bitops_uniform.rs",
            "bitops.rs", "zero_dim_la.rs", "jv_sieve.rs", "jv_sieve_graph.rs", "jv_sieve_two_large_prime.rs",
            "jv_sieve_two_large_prime_dlp.rs", "jv_counted_linear.rs", "jv_homogeneous_quotient.rs",
            "jv_extension_remainder_dlp.rs", "jv_principal_relation.rs", "large_prime_filter.rs",
            "large_prime_filter_mst.rs", "jv_large_prime_rank_bound.rs"]:
    copy("cryptanalysis/" + rel)
assert hashlib.sha256(open(f"{C}/zero_dim_la.rs", "rb").read()).hexdigest() == \
    "5ae85b3458c92dd4350604830966b1a83b240e2340f584e976ae2516ac4a841d"
ov = open(OVERLAY, "rb").read()
assert hashlib.sha256(ov).hexdigest() == "efc554a4a7f7d8e84d06bf3860c7ef99a294721103a041b4a4dd314c30a60f17"
open(f"{C}/jv_cover.rs", "wb").write(ov)

ops = open(f"{F}/prime_hyperelliptic/ops.rs").read()
a = ops.index("/// Words a hash of a divisor absorbs")
b = ops.index("/// Everything this thread has tallied, by phase.")
ops2 = ops[:a] + ops[b:]
open(f"{OUT}/src/prime_hyperelliptic/ops.rs", "w").write(ops2[:ops2.index("#[cfg(test)]")])
open(f"{OUT}/src/prime_hyperelliptic/mod.rs", "w").write("//! Shim: only ops.rs (divisor_hash_words removed).\npub mod ops;\n")

fx = open(f"{F}/cryptanalysis/fx_hash.rs").read().split("\n")
keep = [l for l in fx[:fx.index("/// A hasher for maps keyed by one monomial mask.")] if "pq_groebner_f2::Mask" not in l]
open(f"{C}/fx_hash.rs", "w").write("//! Shim: fx_hash.rs up to the Mask hashers (byte-copied).\n" + "\n".join(keep) + "\n")

GC = {"mm", "am", "sm", "UPoly", "PolyRing", "rref_mod_p", "SparseRel", "square_core", "wiedemann_u64", "SquareCore",
      "trim", "berlekamp_massey_u64", "equal_solver_words"} | set(os.environ.get("GC_EXTRA", "").split())
gc_head = open(f"{F}/cryptanalysis/gaudry_cubic.rs").read().split("\n")
uses = [l for l in gc_head[:120] if l.startswith("use ") and "residual_walk" not in l]
open(f"{C}/gaudry_cubic.rs", "w").write(
    "//! Shim: byte-copied items of gaudry_cubic.rs (frozen fcda1e22).\n#![allow(dead_code, unused_imports)]\n"
    + "\n".join(uses) + "\nuse super::residual_walk::{inv_mod, is_prime_u64, mix64, pow_mod};\n"
    + extract(f"{F}/cryptanalysis/gaudry_cubic.rs", GC) + "\n")
RW = {"pow_mod", "inv_mod", "is_prime_u64", "mix64", "sqrt_mod", "mul_mod", "RelationSystem", "FixedFactor", "Strategy", "add_mod", "sub_mod", "SHOUP_MAX_MODULUS"} | set(os.environ.get("RW_EXTRA", "").split())
open(f"{C}/residual_walk.rs", "w").write(
    "//! Shim: byte-copied items of residual_walk.rs (frozen fcda1e22).\n#![allow(unused_imports)]\n"
    + "use std::hint::select_unpredictable;\nuse serde::Serialize;\nuse super::jv_bitops;\n"
    + extract(f"{F}/cryptanalysis/residual_walk.rs", RW) + "\n")
open(f"{C}/glv_invariant_base.rs", "w").write(
    "//! Shim: byte-copied items of glv_invariant_base.rs (frozen fcda1e22).\n#![allow(unused_imports)]\n"
    + "use crate::cryptanalysis::residual_walk::{inv_mod, is_prime_u64, pow_mod, sqrt_mod};\n"
    + "use rand::rngs::StdRng;\nuse rand::{Rng, SeedableRng};\nuse std::collections::VecDeque;\n"
    + extract(f"{F}/cryptanalysis/glv_invariant_base.rs", {"poly_roots_fp_counted_until", "poly_gcd", "poly_trim", "mulm", "negm", "poly_powmod", "subm", "addm", "poly_mulmod", "poly_rem", "poly_divrem", "poly_mul", "poly_sub", "poly_add", "poly_monic", "poly_eval", "poly_deg"}) + "\n")
open(f"{C}/groebner_f4.rs", "w").write(
    "//! Shim: byte-copied items of groebner_f4.rs (frozen fcda1e22).\n"
    + extract(f"{F}/cryptanalysis/groebner_f4.rs", {"Ordering", "cmp_monomial"}) + "\n")
open(f"{C}/ic_measurement.rs", "w").write(
    "//! Shim for ic_measurement::callgrind_dump: inert outside Valgrind.\n"
    "pub(crate) fn callgrind_dump(_label: &'static [u8]) {}\n")
mods = ["bitops", "jv_bitops", "f4_fp", "jv_cover", "fx_hash", "gaudry_cubic", "glv_invariant_base",
        "groebner_f4", "ic_measurement", "residual_walk", "zero_dim_la", "jv_sieve", "large_prime_filter"]
open(f"{C}/mod.rs", "w").write("#![allow(dead_code, unused_imports, unexpected_cfgs, clippy::all)]\n"
                               + "".join(f"pub mod {m};\n" for m in mods))
open(f"{OUT}/src/lib.rs", "w").write(
    "//! Minimal local build of the Root41-equivalent closure (l1-steps-2-3 lane).\n"
    "#![allow(dead_code, unused_imports, unexpected_cfgs, clippy::all)]\npub mod cryptanalysis;\npub mod prime_hyperelliptic;\n")
os.makedirs(f"{OUT}/docs/units", exist_ok=True)
for v in ["bitop-v1.vectors.json", "bitop-v2.vectors.json"]:
    shutil.copyfile(f"/home/user/crypto/docs/units/{v}", f"{OUT}/docs/units/{v}")
print("assembled", OUT)

# The 11c9a031 deferred-pair modules, under the same module names and
# attributes as Run50's examples/jv_sieve_deferred_pair_compare.rs, so that
# `super::` and `crate::cryptanalysis::` resolve exactly as in that example.
R50 = f"{SCR}/run50src"
os.makedirs(f"{OUT}/src/run50")
mods50 = [("deferred_pair_graph", "jv_deferred_pair_graph", "#[allow(dead_code)]\n"),
          ("extension_deferred_pair_dlp", "jv_extension_deferred_pair_dlp", ""),
          ("extension_quadratic_pair_dlp", "jv_extension_quadratic_pair_dlp", ""),
          ("extension_sparse_dlp", "jv_extension_sparse_dlp", "#[allow(clippy::type_complexity)]\n"),
          ("homogeneous_quotient", "jv_homogeneous_quotient", "#[allow(dead_code)]\n"),
          ("principal_relation", "jv_principal_relation", "#[allow(dead_code)]\n#[allow(clippy::too_many_arguments)]\n"),
          ("quadratic_pair_graph", "jv_quadratic_pair_graph", ""),
          ("quadratic_pair_relation", "jv_quadratic_pair_relation", ""),
          ("quartic_discriminant", "jv_quartic_discriminant", ""),
          ("sparse_collection_rank", "jv_sparse_collection_rank", ""),
          ("sparse_kernel", "jv_sparse_kernel", "#[allow(dead_code)]\n")]
body = "//! Run50 (11c9a031) deferred-pair modules, byte-copied, mounted as in its example.\n"
for name, file, attrs in mods50:
    shutil.copyfile(f"{R50}/{file}.rs", f"{OUT}/src/run50/{file}.rs")
    body += f"{attrs}#[path = \"{file}.rs\"]\npub mod {name};\n"
open(f"{OUT}/src/run50/mod.rs", "w").write(body)
lib = open(f"{OUT}/src/lib.rs").read()
open(f"{OUT}/src/lib.rs", "w").write(lib + "pub mod run50;\n")
print("mounted run50 modules")
