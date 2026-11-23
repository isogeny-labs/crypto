# Rational point-halving auxiliary setup: prospective software qualification

2026-11-15 UTC. This protocol precedes all new source and native calls. The
parent implementation is PR4300, with the existing fixed F19 recovery and
102/2 local Lean gates. This continuation replaces its preset setup in a
bounded software control. General-field execution, growing-family degree/yield
and the complete subexponential objective remain open. [pcc:unverified]

Sources checked (KB unavailable): parent SETUP-REVIEW.md and its linked
auxiliary extension/primary modular-curve review; official MIT18.783 Fall2023
Lecture2 section2.2 and Lecture3 sections3.11–3.11.2, opened before this choice;
FLINT's polynomial API; actual native Curve/Tower/roots/collector/replayer
interfaces. scripts/research-kb status returns127, no executable; no loaded
KB tools. No exhaustive prior-art or novelty claim. Conductor is unavailable
on PATH; no successful reservation is asserted.

Implementation scope: this study; parameterize the existing auxiliary Tower
constructor and reference fibre by public A,T,G, preserving their defaults;
reuse the same GraphOracle, projected equations, checked lifting, collector
and cold replayer. All research arithmetic/solvers/qualification/replay stay
C++/Rust. Shell/Python only perform official tooling, metadata and rendering.
No new scientific timing, paid run, statistical performance or leaderboard
result is authorized. These are bounded synthetic functional controls.

For a finite nonzero P=(xp,yp) on a nonsingular short curve, find candidate
halving abscissas using the degree-four equation obtained by clearing the
standard doubling denominator. Lift all abscissas with the existing native
quadratic roots interface; retain exactly the canonical points H satisfying
[2]H=P. Return every such H once. Check primes5..257, canonical coefficients,
curve nonsingularity, canonical on-curve P and every returned witness. Poles
and x-coordinate roots alone are not accepted as halves. Retain an evaluated
identity.certificate/v1 for the cleared polynomial using the existing native
issuer/checker:32 author points and128 extra points, seeds1234/424242.
This certificate alone is neither an integer identity proof nor program
correctness. [pcc:unverified]

The order8 sampler draws a,x0 uniformly in Fp using rejection from64-bit
outputs of std::mt19937_64, sets b=-x0^3-a*x0, rejects singular curves, and
starts at K=(x0,0). Twice choose uniformly from the complete rational-halves
set, abandoning a dead path. Check [8]T=O and[4]T!=O. At most4096 curve
trials; no point count or field/point census in this candidate. Retain all
samples, rejected draws, paths, half counts and selections. Setup RNG seed1234.

For each successful T, allow at most64 accepted auxiliary-point proposals.
Each uses at most128 uniform proposals u in[0,2p], with O at2p, otherwise
x=floor(u/2), a canonical root sign, rejecting nonsquares and the duplicate
sign at y=0. Test trace collision exactly via the existing Tower terminal
[2]phi(G)=O criterion. No [2d]G substitute. Retain every proposal and rejected
trace. The cold-control sampler also requires exactly3 canonical main-base
points, constructed from the actual fibre by quadratic roots on main E/F19.
This public-curve/base-count filter precedes all target queries. Its acceptance
probability is not borrowed from the unfiltered subgroup sampling bound.
Caps are not retuned after outcomes.

Freeze one qualification001 round:

1. Native admission/identity/old-default structural controls. Complete halving
   census on every nonsingular short model and every finite point for
   p in{5,7,11,13,17,19}, with the census confined to the outer control driver.
   Independent OpenSSL enumerates the same domain, computes all doubling
   fibres, and compares every complete returned set. Stop on disagreement.
2. One order8 plus trace-good setup at each
   p in{19,23,31,43,59,83,127,251}, seed1234, unchanged caps. Independent
   OpenSSL replays all decisions/halves and exact order, trace exclusion and
   eight actual fibre abscissas. Every failed search is UNKNOWN and retained;
   no replacement seed or field is allowed. These fields do not satisfy the
   large-parameter counting theorem and are not a scaling dataset.
3. Rebuild the legacy default F19 cold path and compare its summary and three
   trace files byte-for-byte against cold006. Recheck its OpenSSL replay.
   No old evidence or binary is overwritten.
4. One generated-setup point-only cold recovery on the same main curve
   E/F19:y²=x³+2, P=(4,3), Q=(18,1), order13. Setup seed1234; separate
   collection stream seed5678. Use the unchanged msolve witness oracle with
   20s CPU/wall,2GiB,one thread,quotient128 and all original/model/curve/fibre/
   group-sum checks. No base logs, known target scalar, main field census or
   norm assignment enters the candidate. Independent OpenSSL validates the
   generated A,T,G/base, every query/relation/kernel and final main answer.
   A failed setup/oracle/certificate/replay stays UNKNOWN; stop on mismatch.

Each outer functional arm has1200s and2GiB. Capture exact source commit,
compiler/dependencies, source/binary/script hashes and arguments BEFORE calls.
Use fresh build/output paths and a separately copied frozen execution script.
Do not edit a running wrapper or overwrite any manifest. Preserve every
input, rejected sample, failure, exit and result. Pack one deterministic whole
raw round with a SHA256 file; use the documented artifact-autofix upload and
full-byte replay for large evidence, never shards. No degree/cap/seed retry
is authorized. Stop if the native or independent representations disagree.

The mathematical next question is the sampler's full distribution and charged
expected work. The existing favorable-model bound uses uniform coefficient
pairs; counting the associated liftable2-torsion pairs and halving choices is
required before carrying it to this sampler. Trace/base filters and finite
caps have their own success factors. No complexity result is assumed here.
[pcc:unverified]
