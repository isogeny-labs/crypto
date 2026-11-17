# Affine multiplication replay: resolved local source closure

The retained dependency source closure now resolves to **132 local prerequisite modules**, followed by **four new calls**, for **136 compiler calls**. Exact archive paths, original hashes, import edges and topological order are in source-closure.json. Resolution uses historical LEAN_PATH precedence and hash-matched archived sources, including name collisions. It uses targeted Git-index filenames from the retained inventory; no broad filesystem scan or download.

The current Lean executable matches the historical executable. Recovered norm custody covers the required standard/Mathlib module imports, including UInt64 Bitwise via the implicit toolchain root. One unavailable legacy import remains: NatClassifier requests Mathlib.Algebra.Field.ZMod. A prospective patch replaces only that import with existing Mathlib.Data.ZMod.Basic, already imported transitively by CanonicalResidue; the proof body stays identical. This amendment requires independent source/import review before execution.

The proposed finite source rebuild is10,800 worker-seconds:900precheck +9,000 shared compiler +900postcheck,1CPU8GiB,no swap,120seconds maximum per module,one manual attempt,no automatic retry. This is a cap, not a success forecast: unmeasured136-call duration may exceed it. Preserve partial outputs and failure if any bound is reached. Do not allocate the sum of per-module maxima.

No allowance is allocated. The inspected root-v3 terminal ledger had30,540 unreserved seconds, but subsequent reservations must be deducted using the coordinator's current ledger. The original campaign ceiling and deadline remain unchanged.

There is no further indefinite search for excluded caches. Next independently review the exact132-module source closure and the single import correction, stage hashes and runtime pins, publish the bounded wrapper, then let the coordinator decide allocation. No axioms, hypotheses, candidate statements, scalar controls or26readback obligations are weakened.
