# Active-call leaf guard: prospective instance gate

Question: does the actual size guard supply the existing finite-precision
theorem's root comparison, without inferring a lower bound from child
shrinkage? This is a source-binding application of an existing Nat theorem,
not a proposed new counting or ECDLP mechanism. No new native run is planned.
[pcc:unverified]

## Sources checked before choosing the application

Read actual `Setup::new`/`Setup::parameters` at crypto76969347 (constructor
bodies unchanged from5ec6aed0), shared `CryptoLeanCore.TaperedPrecision`
at crypto-lean50b95696, the pinned primary AppendixD.11 and the existing
October30 contraction-cost review. The latter already uses an active-call
leaf threshold in its inherited-precision argument. Read the exact official
Lean4.35 theorem `Nat.div_lt_iff_lt_mul` in
`Init/Data/Nat/Div/Basic.lean:314`: for positive q,
`s/q < M ↔ s < M*q`. This is the owning theorem; do not rederive it.
[pcc:unverified]

KB MCP absent; two wrapper searches (`active call leaf cutoff inherited
precision root comparison`, `hroot lower child size leaf cutoff`) and theorem
lookup each return127. Bounded GitHub searches in crypto and crypto-lean find
source pointers but supply no absence or novelty claim. Exact source reads
control the application. [pcc:unverified]

## Six fields, before the first Lean edit

1. Structure: ordinary Nat division/order with positive divisor. No custom
   graph, field, network or arithmetic wrapper.
2. Existing result: `Nat.div_lt_iff_lt_mul` as stated, followed by
   `Nat.le_of_lt`. For the actual q=max(k,1), existing max order supplies q>0.
3. Instance: `Setup::new` stores leaf_size=s/q with q=max(floor_sqrt(lambda),1).
   The first `parameters` return rejects M<=leaf_size. Every call reaching
   later active tests therefore has s/q<M. This source reading is not a
   machine-checked Rust refinement, and reaching this guard alone does not
   establish successful Setup/parameter admission.
4. Assembly: compile the existing exact shared scalar module unchanged into
   a private import directory; apply the Std division theorem and positive-max
   result; instantiate the shared precision theorem with the derived hroot.
5. Missing generic components: none. Keep existing owner arithmetic/proofs
   unchanged; only a consumer application is needed. No enlarged limit,
   custom axiom, native_decide, new recurrence or replacement division proof.
6. Inheritance: division/strict comparison from Std; positivity from max;
   weakening from lt to le; all precision arithmetic from the existing
   `CryptoLeanCore.TaperedPrecision.childPrecision_mul_le` theorem.

## Written implication and remaining premises

For q>0, s/q<M implies s<Mq by the existing theorem, hence s<=Mq.
This holds for any k used in q=max(k,1); it does not require separately
reproving that k is the square root. The actual source uses floor division
for the leaf threshold. The paper's rounded-up threshold is separately
identified and is not substituted into the source expression. [pcc:unverified]

In the precision application preserve every other premise: u<=M, i<u,
inherited p<=c*s*2^A, c*q*u<=2^(u+1), and
8u(A+2u+1)<=2^(A+u+1). Deriving hroot alone discharges none of these.
The conclusion remains conditional34. It does not strengthen the previous
finite root35 theorem or establish complete histories/bit cost. [pcc:unverified]

## Controls and acceptance, before execution

Require exact unchanged owner source, consumer compilation, all explicit
axiom queries, the unchanged proof gate, and official fresh replay. Retain
commands, sources, hashes and every failure. Standard foundations only.
Include kernel-decided small controls: dropping activity can fail (s3,M1,q2);
allowing divisor0 can satisfy the divided guard while falsifying the product
comparison; an inactive divisible boundary can still satisfy the weaker
non-strict comparison, so do not claim the weaker comparison is equivalent
to activity. These controls illustrate premise scope, not the universal proof.
[pcc:unverified]

Use official Lean4.35.0, Std-only imports and the exact shared source at50b95696.
One compiler worker,600s phase ceiling, unchanged sampled whole-cgroup stop
15032385536bytes. Local only; S3 excluded. This study makes no timing,
production recursion or new native-execution claim. Publish source-linked
proof/status/visual evidence to the existing in-progress PR5046 after checks.
[pcc:unverified]
