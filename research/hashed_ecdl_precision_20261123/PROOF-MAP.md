# Sharp inherited precision: instance gate and proof plan

Scope: a source-bound quantitative refinement of the already published D.11
argument. It is not a new contraction algorithm, full implementation proof,
or asymptotic improvement. Existing generic Mathlib laws are used as stated.

Primary source: openai/math at42907c9e999cc00d6abe7a50bfc1023250edd1b9,
build/quantitative-contraction.tex blob51ba54efc29f51bbc0426f848003030da484f354.
Native source: crypto main816f9c3a / qualified HEAD1f719877; no contractor
behavior change is planned. Active continuation: PR4996.

## Six fields before any Lean edit

1. Structure above: Mathlib ordered field real numbers, finite sums on
   Finset.range, natural-number powers/division, and their standard casts.
   Source inputs are existing Nat/Real values and functions, not a new syntax,
   automaton, wrapper or cost-algebra structure.
2. Laws used as stated: `le_geom` (Mathlib.Analysis.SpecificLimits.Basic),
   `geom_sum_Ico_le_of_lt_one` (Mathlib.Algebra.Order.Field.GeomSum),
   `Finset.sum_le_sum`, `Finset.mul_sum`, `Nat.div_mul_le_self`,
   `Nat.lt_two_pow_self`, `Nat.pow_le_pow_right`, and ordered multiplication.
   These supply geometric shrinkage, finite summation and exact scalar bounds.
   No replacement proof of those laws is permitted here.
3. Object as instance: source call history j<d with edge counts M_j, scales
   u_j>=1, cumulative exponents a_j, active valuations i_j<u_j, root M_0<=s,
   8*M_{j+1}<=M_j, initial precision p0<=s. The precision increment is the
   literal native expression 8*ceil(3M_j/u_j)*2^(a_j-i_j)+8*M_j*(a_j+1).
4. Assembly probe: type-check the two named geometric laws at ratio1/8 and
   the native natural-number-to-real edge-count sequence. Then instantiate
   finite-sum monotonicity at the native increments. Probe output must precede
   substantive theorem edits; no sorry, custom axiom, native_decide or raised
   elaboration limits. Existing original Lean4.35 kernel, one worker only.
5. Missing components only: instantiate integer ceiling/range inequalities;
   bind cumulative exponent monotonicity and exact native precision; derive
   the explicit inherited coefficient; expose and check scalar absorption
   premises that turn it into a sharp MD/u bound. General path extraction
   from Rust execution, full allocation/entry complexity and asymptotic
   absorption from all source guards remain separate obligations.
6. Inheritance: real ordered field via Mathlib instance; Nat field embeddings
   via standard casts; edge shrink via `le_geom`; finite geometric budget via
   `geom_sum_Ico_le_of_lt_one`; finite accumulation via `Finset.sum_le_sum`;
   ceiling product via `Nat.div_mul_le_self`; a+1<=2^a via
   `Nat.lt_two_pow_self`; exact code formulas via equality of written scalar
   expressions, with no claim of a verified Rust compiler/refinement.

## Dependency order and statement ownership

- Gate probe: anonymous applications of existing laws, not a new law.
- Native inherited-precision instance obligation: apply the laws to the literal
  source increments. The target explicit coarse form is81*s*2^A when all prior
  a_j<=A. This replaces the source's unspecified polynomial factor with a
  deliberately conservative finite coefficient; it is a proposed bound until
  its theorem and source fidelity have been checked.
- Native sharp endpoint: use the inherited result, active a=A+2u, i<u,
  above-leaf s<=M*q, and explicit scalar conditions81*q*u<=2^(u+1) and
  8*u*(A+2u+1)<=2^(A+u+1). Target u*p_t<=34*M*D. Neither absorption condition
  assumes a precision bound; both concern independently specified dimensions.
- Only after proof: native diagnostic binding/control if a missing reusable
  source utility is needed. Do not rerun previous245 controls for another green.

No generic missing law has been identified: every listed law already belongs
to Mathlib. If a new reusable law becomes necessary, stop that study proof,
build it generically in crypto-lean first, and update this map before edits.

## Proposed inequality chain (not yet proved here)

For prior calls with positive u_j<=M_j, h_j*u_j<=4M_j, hence h_j<=4M_j.
Also D_j<=2^a_j and a_j+1<=2^a_j, so delta_j<=40M_j*2^a_j.
`le_geom` and the finite geometric-series bound give sum_{j<d}M_j<=2s.
Therefore p_d<=s+80s*2^A<=81s*2^A.

At the current call, D=2^(A+2u-i)>=2^(A+u+1). The first scalar absorption
condition and s<=M*q imply u*p_d<=M*D. The second gives
8M(a+1)u<=M*D. Exact ceiling arithmetic gives8hDu<=32MD. Adding yields34MD.

## Fidelity and unfinished bridges

These endpoints quantify over supplied numerical histories satisfying source
shape/shrink hypotheses. They will not by themselves prove that every Rust
execution has such a history, source asymptotic admission, weighted tuple
coverage, total construction/entry work, or polynomial retained bytes.
The nonzero public recursion remains unrepresentable on the current64-bit API.
Do not call a symbolic scalar check an allocated recursive contraction.

Pinned proof runtime: Lean4.35.0 commit eb6cafb7290e8bd5f2937522d8d804f58f02bafc;
cached Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679. Crypto-lean inspected
atff61caed5cd414cf133e110bdf5740356de1dd54; it pins a different Mathlib
cfc55fc58714a59dc06e6e79edc6d1d95140d3c2 on the same toolchain. No crypto-lean
declaration is imported for this Mathlib-only assembly, and no dependency pin
will be silently changed. The library's current catalogue contains no matching
precision resource lemma; absence is limited to that inspected catalogue.

Resource plan: existing machine only, USD0 extra spend, no cloud/S3;
one Lean worker,600-second original per-phase cap, whole-cgroup stop at
15,032,385,536bytes. No research performance inference from proof times.

### Gate refinement after actual infrastructure observations

The first copied probe was outside Lean's current package root; the original
exit1 is retained. Running from the copied input directory fixes that root
binding. The original Real/analysis import probe then reached the managed
512MiB interpreter limit (exit134), without the physical stop triggering.
No theorem result follows from either attempt.

Use the smaller existing `Monotone.seq_le_seq` in Mathlib.Order.Iterate
directly, and instantiate the finite field/geometric laws at exact rationals.
This eliminates a topology/Real dependency not needed for integer source costs.
All natural-number values embed into the ordered field ℚ without loss; the
source claim and scalar conditions are unchanged. Field1 is therefore ℚ;
field2 uses `Monotone.seq_le_seq` instead of its Real specialization `le_geom`.
The new anonymous assembly probe is ProbeQ.lean. No generic law is re-proved,
no library pin changes, and the managed512MiB/physical guards remain unchanged.

The smaller-import rational probe also failed during interpreter loading with
the managed512MiB exception (exit134); no theorem diagnostic appeared. Its
whole-cgroup sampled peak was14,531,309,568bytes, below the unchanged physical
guard. Preserve that failed attempt and the managed512 launcher. The next setup
uses `-M0` with the same whole-cgroup physical stop and600-second deadline;
no elaboration heartbeat/recursion limit changes. This changes an internal
loader limit, not the physical compute envelope or mathematical hypotheses.

### Passed assembly and ownership refinement

01f passed the rational Mathlib assembly;02 passed a separate Std floor probe.
The previous01d/01e typing/import diagnostics remain retained, not proof passes.

The scalar proof is reusable across the native counter's consumers. Its owner
is therefore the shared library, not a copied study lemma: add a Std-only
CryptoLeanCore.TaperedPrecision module at crypto-leanff61cae before its native
consumer. It introduces the source precision expression by composition of Nat
addition/multiplication/division/power, and a scalar bound with a variable
inherited coefficient c. Natural arithmetic laws come from Std as stated.
No hand-rolled ordered field, finite-sum/geometric law, wrapper state machine
or changed library dependency is needed. Std-only compilation on the exact
shared Lean4.35.0 pin does not import either Mathlib revision.

The shared scalar law takes the explicit inherited bound as a hypothesis;
it is NOT a proof of the full path bound. The written81*s*2^A path argument
and its Mathlib assembly are retained, with the full scalar-history binding
still open until separately formalized. Source-specific test instances and
premise-deletion counterexamples belong to the native study, not the library.

The scalar statement is universally quantified over natural dimensions. It
assumes u<=M, i<u, s<=M*q, p<=c*s*2^A, c*q*u<=2^(u+1), and
8*u*(A+2u+1)<=2^(A+(u+1)). Its conclusion is
childPrecision(p,M,u,A+2u,i)*u<=34*M*2^(A+2u-i).
The6 source/scalar premises are independent of that conclusion. The controls
include a valid positive numerical instance and failures on deleting active
valuation, width, inherited budget or absorption premises. These are scalar
instances, not public admitted recursive graphs or ECDLP solves.

### Repository gate resource observation

06-pcc-lean-verifier was interrupted by the unchanged whole-cgroup physical
guard: peak15,044,296,704bytes, exit143, no returned verdict. This is an
incomplete gate execution, not a theorem failure. No physical limit was raised.
The exact generated external Mathlib `.ilean` editor indexes were archived
losslessly and byte-compared before compacting their uncompressed cache copies:
8,643files,289,914,149bytes into43,482,563bytes. Source, runtime/proof artifacts,
research records, checkpoints and native executables are unchanged. Restore
instructions and the archive/manifest digests are recorded. This reduces fixed
container footprint before a separately named gate attempt; no mathematical
condition, proof-gate code or trust list changes.

07 also hit the same guard (15,032,705,024bytes, exit143, no verdict); retain
it separately. A bounded3,708-file external Mathlib private-cache subset was
then archived losslessly (1,499,860,416bytes into482,054,328bytes), fully
byte-compared and rehashed before compacting uncompressed duplicates. This
changes availability of external cached Mathlib artifacts until restoration,
not their bytes; all owned research/proof/checkpoint files remain unchanged.
Do not run a Mathlib build before restoring the explicitly documented caches.
The Std-only third gate attempt retains the identical source, declaration
hash, proof gate, trust list, physical limit and deadline.

### Final focused outcomes and claim fidelity

09-pcc-verifier-after-bounded-cache-compaction passes (exit0, no physical
stop, sampled whole-cgroup peak14,011,281,408bytes). It binds the exact
declaration SHA25672b48b96db59bc71d059360987abec830ef5c8019625d01758dbd3ba1e7034c1
and invokes the unmodified native proof gate. The raw verdict is an unsigned
local `pcc.verdict/v1`; claim75fa8c has no signed ledger verdict.
10-native-controls-gate passes7theorems with0trusted entries. The3 shared
statements and7 controls have the explicit axiom output from05. No theorem
premise, code, proof gate or trust list was changed between06,07 and09.

Shared source is now committed atcrypto-lean3e47fb776a4eb5231de4c8aad2bb6e756c2f89a1
(draft PR35), blob42ed02cde305b5df5c8145cff3e83df653580831. Native formal
qualification copies are byte-identical frozen dependency source, not a new
study implementation of the library's laws. Local scalar/source/control
review found no blocking issue; committed native publication binding remains
a separate addendum. Full shared-library Mathlib/current-root CI is open.

The81 history coefficient is only for P0<=s and M0<=s normalization (native
nonempty s=2E,P0=E+1), plus path shrink. Source P0<=C*s alone does not fix81.
Neither the scalar theorem nor the passing probe proves the complete history
binding. All claims are reported at this explicitly conditional strength.
