#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Schur construction panel: launch and custody
Running experiment snapshot · 2026-11-01T20:42:23.042429+00:00

The frozen Schur panel is executing on its original producer. This observation
contains 17 complete context rows: 15 checked Schur constructions
and 2 certified leading-block obstructions. These are prefix observations.
No complete-panel or export result exists at this snapshot. The postprocessor
follows this same invocation.

#image("progress.svg",width:100%)
Raw source: runs/cases.jsonl, first 245249835 bytes. The append-only prefix hash
and complete row IDs are in snapshot.json. This graph counts completed branches,
not statistical success estimates or a full recovery result.

*Full scope.* Each arm retains all 81 source contexts, using their balanced
leading block, and adds 240 synthetic contexts. Synthetic cases cross primes
7, 13, 19, 43; dimensions 2, 3, 4, 7, 12; cuts 1, floor(N/2), N-1 with
duplicates removed; and regular, zero-Schur, singular-Schur, invertible-full
with blocked-leading, and zero-matrix types. Two arms give 642 rows. All
leading obstructions remain in scope, including 96 synthetic rows whose
full matrix is invertible despite a singular leading block.

*Checks.* Exact reference elimination supplies a two-sided leading inverse
or a nonzero leading nullvector. With an inverse, check the full Schur
displacement, width s+2, all nested source and oracle counts, and both product
directions through the Schur wrapper and generated companion operator. With
a blocked inverse, generation and application must raise the specific
obstruction and retain partial work. That obstruction does not establish
full-matrix singularity. Equal-wrap, output, count and generator controls
are fixed before data.

*Custody.* Predata commit f5cac20. Protocol hash:
7403509a477e478f03427db64fd4c864e8e62ea99dba3f5d84f8bcd249204480.
All 642 execution-row seeds matched before data. Eleven source files and
one public input archive are locked. One fresh timeout 3600 invocation;
no restart or extension. Producer PID 2104676, session 68027; native timeout
PID 2104675. Postprocessor PID 2106367, session 68331. launch.json records
start ticks and a deadline estimate; the existing native timeout controls
the actual deadline.

*Costs.* Generator construction schedules 2s+6 full-source applications and
s+2 leading solves per direction. The reference inverse oracles record dense
dot-product terms and output reductions separately from source and native
work. Those calls are not free or evidence of fast inversion. No clock,
CPU or RSS comparison is claimed. All failures and partial rows are retained.

*Next action.* Follow the existing processes. After producer exit 0, the
postprocessor runs the frozen streaming analysis and full-file export check
once. A missing owner without an exit receipt is an interruption, not a reason
to rerun. After terminal checks, render and inspect a fresh final report and
seal all raw records.

#pagebreak()
= Derived next adapters: RHS and primal/dual lifting
Written and syntax-checked only. No adapter numerical trial ran in this round.
Source: companion-schur-lift/lift.py; proof: companion-schur-lift/DERIVATION.md.
These files do not alter the running Schur instrument.

For A=[[E,B],[C,D]] with E invertible, S=D-CE⁻¹B, and b=[b1;b2],
set h=b2-CE⁻¹b1. Given any trailing vector z, define
x=[E⁻¹(b1-Bz);z]. Then the exact residual is
Ax-b=[0;Sz-h]. Thus a checked Schur solution lifts to a full-system solution.

For any trailing v, define w=[-E⁻ᵗCᵗv;v]. Then
wᵗA=[0,vᵗS] and wᵗb=vᵗh. In particular, vᵗS=0 and vᵗh=1 produce
a normalized full-system exclusion witness. The existing separate feature
annihilation check remains necessary; a Gram-null identity alone does not
establish that feature-level condition.

Each adapter uses one full-source application and one leading-block solve.
Additional scalar work is m subtractions for RHS reduction and n for either
lift, where n=dim(E),m=dim(S). Dual lifting uses the transpose direction.
The helper records those calls through the Schur object's counters and lets
LeadingBlockObstruction propagate unchanged.

The next adapter gate must check both residual identities, normalized dual
witnesses, changed RHS/sign controls and complete nested costs, after the
current Schur construction panel is reconciled. Then a recursive backend
must replace the explicit leading-block reference oracles. No recursive
solver or new full ECDLP recovery is claimed by these unexecuted adapters.
