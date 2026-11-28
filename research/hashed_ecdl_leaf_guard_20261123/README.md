# Bind the active leaf guard to the precision comparison

The actual source size guard supplies one premise of the existing finite
precision theorem. For natural source size s, current edge count M, and
positive q, Std's existing division equivalence gives

```
s / q < M  =>  s < M*q  =>  s <= M*q.
```

`Setup::new` uses q=max(floor_sqrt(lambda),1) and stores leaf_size=s/q.
`Setup::parameters` rejects M<=leaf_size before its later active tests.
Thus a call passing this size guard has the root comparison hroot used by
the existing precision law. This is a direct application of the existing
Std theorem, with source binding; no new generic division theorem is needed.
Formal record **PCC-20261123-85abe4** remains unsigned. [pcc:unverified]

The [consumer](formal/LeafGuard.lean) then applies
`CryptoLeanCore.TaperedPrecision.childPrecision_mul_le` unchanged. Its other
premises remain explicit:

```
u<=M, i<u,
p<=c*s*2^A,
c*q*u<=2^(u+1),
8u(A+2u+1)<=2^(A+u+1).
```

Here p is ring precision, not the elliptic-curve field modulus. The conclusion
remains `childPrecision(p,M,u,A+2u,i)*u <= 34*M*2^(A+2u-i)`.
The new application discharges hroot alone. It neither assumes a lower
child size from the source's independent shrinkage upper bound nor discharges
inherited precision and the dimension conditions. The earlier conditional34,
finite-history and root35 results remain unchanged. [pcc:unverified]

## Qualification and scope controls

One source-frozen qualification compiles the exact unchanged shared owner
privately, then the consumer. Unchanged proof gates pass seven consumer and
three existing owner declarations, with zero trusted native proofs. Official
`leanchecker --fresh LeafGuard` exits0. Eight explicit axiom queries use only
propext/Quot.sound; the three literal controls use no axioms. Official Lean4.35.0
and Std-only imports are used, with no Mathlib dependency in this focused
application. The existing local PCC verifier passes both declaration bindings and its
proof-gate checks; [phase02](evidence/02-formal-pcc/stdout.txt) retains the output.
Signed standing remains unverified. These are unsigned producer checks. [pcc:unverified]

| Scope control | Kernel-checked fact |
| --- | --- |
| Drop activity | s=3,M=1,q=2: the guard is false and s<=Mq is false |
| Drop divisor positivity | 1/0<1 holds in Nat, but 1<1*0 is false |
| Inactive divisible boundary | 4/2=2; 4<=2*2 holds, while 4<2*2 is false |
| Nonempty conditional application | p=1,M=16,u=12,A=i=0,s=24,k=2,c=1 satisfies all displayed scalar premises |

The boundary control prevents falsely calling the weaker non-strict comparison
equivalent to activity. The last witness establishes a nonempty arithmetic
application; it is deliberately not Setup-admitted production recursion.
The universal result follows from the existing division/precision theorems,
not from the small controls. No new native or solver run is added.
[pcc:unverified]

Manually fabricated Setup values in forced tests are not assumed to satisfy
Setup::new's leaf invariant. The natural-number application is not a full checked Rust refinement or a
production-admission theorem. Passing the size guard alone does not supply
degree, scale, array, overflow or other later checks. In particular current
64-bit usize still has root_u0. The paper's rounded-up leaf threshold is
identified separately from the implementation's exact floor division.
[pcc:unverified]

## Sources and reproduction

The [prospective six-field map](PROOF-MAP.md) precedes the first Lean edit.
The standard owner is `Nat.div_lt_iff_lt_mul` at official Lean4.35,
`Init/Data/Nat/Div/Basic.lean:314`; the precision owner is unchanged at
crypto-lean50b95696. Actual source is crypto76969347,
`crates/hashed-ecdl-tensor/src/contraction/recursive.rs`, Setup methods.
Pinned primary AppendixD.11 and the October30 contraction-cost review already
use active leaf thresholds in the inherited-precision argument. This is
source-level assembly of established mathematics. [pcc:unverified]

KB MCP was absent; two exact mechanism/negative searches and theorem lookup
each returned127. Bounded repository searches plus exact source reads provide
no novelty or absence clearance. All source/command/output identities are
retained in [SOURCE-BINDING.json](SOURCE-BINDING.json), the qualification
receipts and the [source-linked PDF](reports/leaf-guard-20261123/report.pdf).
One worker,600s phase ceiling and the unchanged15032385536-byte sampled
cgroup stop were used; qualification peak14373085184bytes. No limit, gate,
owner statement or theorem premise was relaxed. [pcc:unverified]

Reproduce with the exact shared source and official Lean4.35, private
`CryptoLeanCore/TaperedPrecision.olean`, and Std-only LEAN_PATH; compile
LeafGuard, run the unchanged gate on both files, inspect all axiom queries,
then run the official fresh checker. Full-library/default-Mathlib integration
of the shared repository and signed standing remain separate open checks.
The next mathematical work must bind inherited budgets and dimensions along
actual source histories and account for complete cost. The port/vertex study,
original native run and recorded CPU comparison stay unchanged. [pcc:unverified]
