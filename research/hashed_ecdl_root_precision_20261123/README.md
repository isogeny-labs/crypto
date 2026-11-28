# Finite precision at the source root

The root precision formula now has a finite35 bound from the source root shape
and array guard. This removes two additional absorption assumptions for this
root case. The existing conditional34 and finite-history81 theorems are
unchanged. Formal claim PCC-20261123-759751 and native control claim
PCC-20261123-4b3d64 remain unsigned. [pcc:unverified]

For natural p,M,u,i, put D=2^(2u-i). The new shared theorem proves the following
bound. [pcc:unverified]

```
p <= M, 32u <= M, i < u
    => childPrecision(p,M,u,2u,i) * u <= 35 M D.
```

The actual root length/array inequality `2^(2u) * 64^(4u) <= M` supplies32u<=M.
The normalized source application uses M=2E,p=E+1,E>0.
The [shared implementation](https://github.com/aburan28/crypto-lean/blob/1c57c092460bee5093072602d3bb058d13bbbcf1/CryptoLeanCore/TaperedPrecisionRoot.lean)
owns these generic laws; this study applies them in
[RootControls.lean](formal/RootControls.lean). [pcc:unverified]

The proof bounds the three terms in four times precision*u by MD,97MD and42MD.
It uses exact Nat floor division and the finite inequalities4u<=2^(u+1) and
16u(2u+1)<=21*2^(u+1). Their sum140MD gives35. The evaluation inequality is
equality at u=3; it is proved by small bases and induction. See the
[proof map](PROOF-MAP-OUTCOME.md) and [diagram](reports/root-20261123/proof-map.svg).
[pcc:unverified]

At u=2,a=4,i=1, the complete normalized identity is

```
2 * childPrecision(E+1,2E,2,4,1) = 546E+2 > 544E = 34*(2E)*8.
```

For the weaker margin domain take E>=32. The full array guard needs E>=2^51.
Thus34 is not a universal constant under these root hypotheses. The prior34
theorem's extra absorption premises fail in this family; it is unaffected.
The primary paper states an asymptotic Big-O envelope with sufficiently-large
conditions, not an explicit universal34 claim. [pcc:unverified]

## Checks completed

| Check | Observed result |
| --- | --- |
| Five shared declarations | Compile; unchanged gate5/5,0 trusted native proofs |
| Seven study declarations | Compile; unchanged gate7/7,0 trusted native proofs |
| Twelve explicit axiom queries | Standard foundations only |
| Official fresh kernel replay | `leanchecker --fresh RootControls`, exit0 |
| PCC statement binding and two proof gates | Local verifier pass; signed standing unverified |
| Native exact root witness | All21 translated scalar guards pass |
| Seven corrupted source fields | All rejected |
| Deterministic weaker-domain sweep | 29,952/29,952 satisfy35;12 exact rational maxima retained |
| Premise-deletion cases | 4864>4480;886>840;2336>2240, with remaining premises checked |
| E=32 abstract34 counterexample | 17474>17408;35 gives17920 |
| Source/binary custody | All pre/post hashes match; one copied read-only native execution |

These are producer observations. The official fresh replay uses Lean's own
kernel; source reviews01/02 are separate written checks by a same-root-team
reviewer, who ran no compiler or native program. Neither constitutes signed
independent execution. [pcc:unverified]

The native witness has M=s=2^262150,p=M/2+1,maximum_degree2,
lambda262150,sqrt512,delta2^235935,u2,a4,length16,array2^48,t2.
The exact equality delta^10=s^9 certifies the floor/ceiling root without a
general root-finding oracle. The leaf cutoff is2^262141; the degree guard
requires only2^236032<=M and the length-array guard2^52<=M. Corruptions change
lambda,u,delta,sqrt,degree,valuation or array one at a time. [pcc:unverified]

This is an **unbounded integer translation** of the scalar guards. It allocates
no graph and is not a production Setup call. Current64-bit usize gives
lambda<=64 and root_u0. The large witness's u2 also fails the paper's
sufficiently-large consequence u>=lambda^(4/5); it is not a counterexample
to that asymptotic statement. The small sweep and deletion cases concern
only the weaker margin domain. Deleting margin also drops the stronger
array guard that implies it. [pcc:unverified]

## Evidence and retained failures

[Native plan](NATIVE-PLAN.md), [raw native output](evidence/phases/07-native-call/stdout.txt),
[formal qualification](evidence/phases/05-full-lean-qualification/stdout.txt),
[axiom queries](evidence/phases/03b-controls/stdout.txt),
[PCC verifier](evidence/phases/08-formal-pcc-verifier/stdout.txt),
[source bindings](SOURCE-BINDING.json), [written review02](reviews/ROOT-MATH-SOURCE-REVIEW-02.json),
and [PDF report](reports/root-20261123/report.pdf) identify the scopes separately.

Retained attempts:01 input outside compiler root;01b missing Std probe name;
02 proof-normalization/API diagnostics;03 import-prefix shadowing;04/04b fixed
memory-stop143. Their attempted sources and outputs remain; later fixes change
no theorem hypothesis, constant, toolchain or memory ceiling. The failed
source copies in `attempts/*.lean.txt` are evidence text, not active proof targets;
SOURCE-MAP pins their original paths and bytes. [pcc:unverified]

The whole-cgroup stop stayed15,032,385,536bytes, with one worker and600s phase
ceilings; the native executable had60s. Lossless archives of completed caches
and duplicate readbacks were compared to the originals before reclaiming space.
All original CPU raw data, qualified binaries, capsules,34/81 proofs and protocol
failures stay unchanged. Local-only artifacts follow the user's no-S3 instruction.
[pcc:unverified]

Sources checked: existing shared core3e47fb77, finite-history study/source audit,
actual recursive.rs at d562832c, and pinned primary AppendixD.11 at
OpenAI/math42907c9e. KB MCP was absent and the wrapper returned127; bounded
GitHub/source fallback supplies no novelty clearance. This result is an explicit
finite resource law, with no claim of a new ECDLP runtime exponent.
[pcc:unverified]

The next mathematical obligation is to bind actual source histories and checked
arithmetic to the cumulative precision/cost laws, including later depths and
rational intermediate widths. Current-main PCC ledger diverges from this
branch at entry1564; both histories are retained, and merge/signed standing
remain open. No canonical ranking or runtime chart changes for this arithmetic
component. [pcc:unverified]
