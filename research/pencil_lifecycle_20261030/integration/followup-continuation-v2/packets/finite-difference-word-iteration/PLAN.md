# Canonical word recurrence composition: prospective plan

T-1525, 2026-11-03. Planning only until coordinator approves validation bounds. No compiler, replay or scientific run has been launched.

## Construction
Use exact published Difference.lean as a dependency, importing its SieveDifference.wordAdd, word_add_canonical, word_add_decode, state_closed and ordered_evaluation lemmas. Define a UInt64 state triple (u,du,v), constant second difference dd=wordAdd p c c and slope f. Initialize (a,wordAdd p b c,e). One step maps (u,du,v) to (wordAdd p u du,wordAdd p du dd,wordAdd p v f), with all three right-hand sides using old state. Iteration is on Nat; runtime counter/iterator refinement is separate.

Premises: 0<p.toNat, p.toNat<=2^63, and each active a,b,c,e,f has toNat<p.toNat. Padding zeros require p>0. No primality is needed for this recurrence theorem. Original full sieve prime/complement/degree premises remain outside it.

## Obligations
1. Prove canonical initialization and constant dd, then preserve canonicality of all state coordinates for every n. Do not assume canonicality of the output.
2. For every n, decode each word coordinate to ZMod p.toNat and prove equality with SieveDifference.state at n. At each step derive non-overflow from both current canonical operands and modulus bound, then invoke word_add_decode. The dd and slope contributions must be joined explicitly.
3. Compose state_closed to obtain quadratic value, first difference and linear value at every n. Derive canonical natural representatives using ZMod.val_natCast_of_lt; use a local NeZero instance from p>0, not a hidden primality premise.
4. Show ordered pre-update observations over List.range count equal the prior ring reference, and obtain equal folds for a common branch-state transition. This is a Lean word-model statement, not compiled Rust semantics.
5. At count=p.toNat, identify the final post-update state separately from the emitted indices0..p-1. Relate its decoded coordinates to the initial values by characteristic-p wrap; retain the actual final unused update.
6. Establish canonical zero/equality correspondence needed to interpret the native ux==0 predicate. Do not infer the rest of the Rust branch semantics or collector correctness from this lemma.

## Source correspondence
native_solver.rs963-968 reads/pads active coefficients;969-972 initializes state and dd;973 iteratesx;974-975 emits current ux/vx;977-979 executes three updates;980 onward uses captured old values even across continues. Lines812-821 implement wordAdd as (a+b)%p. Native mul/peval/complement and full-loop/range compiler behavior are unchanged, not newly formally verified. Array slice validity and degree0..2 are source-side premises; actual native caller filters degree2. Domain p<=2^63 matches the existing candidate bound.

## Proposed finite ordinary Lean validation
At most THREE prospectively frozen attempts, each120seconds including compilation and readback, total360seconds maximum. Each uses8GiB RLIMIT_AS, Lean -M3500 -j1 -s16384, OS10MiB stack; soft work cutoff110seconds with kill/reap reserve inside120. Copy exact imports into private workspace as needed; inventory original dependency/toolchain files before/after and retain unchanged-source hashes. No scientific worker charge or new runtime admission is inferred. No retry beyond three or360seconds without new prospective amendment. Root must approve this proposal before first compiler invocation.

Each attempt freezes exact source and runner before execution. Compile Difference and new WordIteration, controls and named axiom readback. Expected-false control is a separate retained invocation within the same outer bound. Reject sorryAx, missing/duplicate names or unknown axioms; allow only propext,Classical.choice,Quot.sound inherited through actual statements. Preserve every failure.

Controls: zero/constant padding, characteristic2 recurrence, zero-valued intermediate state, late-step/wraparound, upper-bound canonical addition and a concrete overflow counterexample when p>2^63. These are word-model controls; no full curve claim. At least one false off-by-one observation/final-update assertion must be rejected.

## Independent review and artifacts
The producer previously reviewed the component and proposed its recurrence; independent proof/source-fidelity reviewer must be a different author. Publish exact producer before formal independent replay. Preserve scoped PDF/SVG/CSV/JSON, commands, receipts, failures and official archived large input/readback logs. No index action until root grant.
