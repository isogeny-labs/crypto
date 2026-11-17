# Vieta timing and proof continuation

The user renewed authorization for half-GCD, an independent degree-bound
audit, general rational towers with charged construction, and then measured
speedup. This is a new prospective timing experiment; the earlier completed
matrices and their frozen protocols/raw evidence remain unchanged.

The merged Rust library already contains opt-in HalfGcd, schoolbook and
Karatsuba/Newton references, generic homogeneous rational towers, full sibling
fiber certification, denominator/pole handling and exact diagonal/identity
branches. Reuse those source bytes rather than claiming another implementation.
Add a separate Rust timing crate and analyzer so the old qualifier and its
source/lockfile manifests remain reproducible.

Timing scope: fresh complete factor-base construction plus one query, with
separate construction/query phases. Every arm constructs the full point base;
lookup additionally builds its hash index. Schoolbook is the reference,
Karatsuba/Newton/classical Euclid is the ablation, HalfGcd is the candidate.
Separate fresh-process HalfGcd A/A and a double-query positive control.
All arms use one binary, one thread, identical budgets and matched targets.
Freeze calibration and primary protocols before their affected data; preserve
all failures/timeouts. Analyze per field/representation/size/population,
including minimum/median, paired ratios, A/A ranges and uncertainty. No pooled
curve-size exponent or whole-DLP inference.

Persistent authorization is isolated local/Git-only, no paid/cloud work or
local S3 operations. Use the official isolation wrapper and persistent bounded
supervision. This host previously lacked PSI and complete affinity control;
new observations determine admission. Below-L2 clocks stay diagnostic with
wall_valid=false; no admitted L2 speed claim follows from an unavailable host.

Proof scope: independently rederive the projector-coordinate identity and
normal-form degree induction, explicitly test full splitting, coprimality,
denominator degree, repeated abscissae, nilpotent/nonsplit slope blocks and
repeated tower fibers. Reuse existing exact checks only as historical evidence.
Seek a mechanically distinct verifier/formalization; do not call another
producer-owned check independent signed standing. Keep the general theorem
open until its actual quantified obligations are checked.

Lean instance gate: the semantic structures are MvPolynomial over a field,
graded constructible subspaces V_T and the finite product of slope algebras.
The needed reusable law is the degree-controlled normal-form implication to
last fall degree. The inspected crypto-lean tree has no matching path; this
is a search observation, not absence of a theorem. Probe the shared library
and its pinned Mathlib before any Lean edit. Missing declarations and actual
build/tool availability must be recorded rather than assuming the conclusion.

Sources checked: FLINT v3.3.1 nmod_poly/hgcd.c and gr_poly/hgcd.c (cached
primary sources), original HKYY constructible-space definitions as cited by
the pinned degree note, PKM section3.3, the existing SPLIT_TOWER_BOUND.md,
DEGREE_AUDIT.md and rational_tower.rs. KB MCP is absent; wrapper status and
two exact mechanism/negative queries exit127. Use primary/internal fallback;
no novelty claim. Shared crypto-lean main uses Lean4.35.0 and pinned Mathlib
cfc55fc58714a59dc06e6e79edc6d1d95140d3c2; local Lean/lake not on PATH.
