# Original-point model adapter — prospective fixed correctness scope

Continue the existing in-progress task and PR5071. Owned paths are
tools/torus-point-solver and this study. Keep all85 earlier source inputs,
archives/claims/raw evidence byte-identical. The one shared-journal conflict
request remains pending; do not edit that conflicted file or duplicate it.

The query takes only public original Spec, R, Q=[8]R and limits. Compile the
unchanged TargetCircuit at arity6/distinct raw indices. Export its original
System variables and sparse polynomial rows directly as finite-field SMT:
one declared variable per original variable, one equality per original row,
all original guards and terms, no extra gate/auxiliary expansion. Validate
modulus/coefficients/monomial indices/degree/size before serialization.

Use a separately versioned native process/model adapter derived from the old
bounded adapter. Admit <=4096 complete bindings, not the old256-input bound;
never bypass or edit the frozen old validator. Request every original variable
only after SAT. Parse exact bindings and field values, check assignment length
and canonical range, run System.check, then TargetCircuit.decode_row with the
unchanged index table and group replay. Retain Unix exit signal separately
from the pipe symptom. UNSAT/unknown/caps are observations/refusals, not absence
or hardness certificates. All producer standing remains unverified.

Development controls:12 known original point assignments taken from the prior
synthetic-model raw archive; strict malformed binding/field/equation controls;
one-wire corruptions and noncanonical representatives; fixed native SMT with
complete known assignments asserted as an **extension control**, never called
root discovery. Native small SAT/UNSAT and output-cap controls. Exactly ONE new
public-input point query at p101/q3/k8,R=(89,27),Q=[8]R, same target as the frozen
spectrum attempt. Do NOT rerun the old spectrum query. No unknown values enter
the new public query. Record checked root or bounded refusal as observed.

Native cvc5 GPL/CoCoA1.4.2 must match SHA
754bc9abb49bcd2ae095bf07a9ba2f68759bf0f232e71f601ac642f802a59fc8.
Use5000ms child limit plus1000ms watchdog grace,2GiB child address space,
stdout<=1MiB/stderr<=64KiB/query<=2MiB, under one final2GiB/600second shell
correctness call. Source commit before qualification; immutable executable,
compiled-in hashes, separate write-once BUILD/PRECALL/POSTCALL; transport
success/collision checked first. No automatic retry of the point query.

These are fixed local development/unit-integration checks under AGENTS §9a,
not a timing comparison, growing-family campaign or paid job. No solver rate,
speedup, strongest-tool or exponent claim from the outcomes. A new comparative
campaign needs a separately frozen full-cost EKS protocol and existing budget.
Keep previous caps/refusals and all earlier route criteria unchanged.

Prior-art review: original torus-s4 System/Poly and torus-coset-circuit
aggregation/index modules, previous model-solver-v1/NEXT.md and native process
code; cvc5 pinned finite-field syntax/docs and CAV2024 BitSplit §§4–5;
https://cvc5.github.io/docs/cvc5-1.4.2/theories/finite_field.html,
https://theory.stanford.edu/~barrett/pubs/OPB%2B24.pdf.
Original complete-addition source cites RFC8032 §5.1.4; it stays unchanged.
No novelty is claimed for the formulas, SMT encoding or classical backend.

KB unavailable: no loaded connector, wrapper127. Intended retrieval: “original
point sparse polynomial finite-field model replay”; “BitSplit direct elliptic
decomposition”; “complete model canonical field representatives”; “solver
timeout versus generic group exponent”. Repository source and primary docs
supply this bounded review; no exhaustive novelty claim. [pcc:unverified]
