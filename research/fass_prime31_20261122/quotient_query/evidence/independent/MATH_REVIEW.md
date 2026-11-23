# Separate-role mathematical and certificate-source review

Date: 2026-11-23 UTC. PR #4966. This review is written analysis and source
inspection; no native build, run, formal proof or signed verdict is reported
here. All conclusions have unverified PCC standing. [pcc:unverified]

## Mathematical scope

I reviewed `THEORY.md`, `PROOF_MAP.md` and
`CANCELLATION_PREDICTION.md`. The cancellation note reviewed has SHA256
`0250383b885c4aaa6a6e3a40f3d4dbe34f49d40adb84f113217b11d27d034d6e`.

| Obligation | Review |
| --- | --- |
| Proposition 1: decomposition certificate | The two cofactor identities and Bezout identity imply equality of the two generated ideals modulo every q. Coprime block product G makes the intersection identity follow from CRT. A positive gcd needs the cofactor identities; Bezout alone would be insufficient. |
| Monic fiber gcd and support | Monicity preserves x-degree on every t root. Divisibility into squarefree split F makes every specialized D squarefree and split. Therefore the exact affine point count is the sum of degree products. |
| Proposition 2: reduced lex bases | For a positive block, leading monomials are x^r and t^s, hence coprime. The t-reduced coefficients and x-monic D make both tails reduced. The pair is a reduced lex basis for that component. |
| Several blocks | Their ideals intersect; taking the union of their bases instead generates a sum. The text correctly avoids calling the union a global Groebner basis. |
| Radicality | The finite selector algebra is a product of copies of Fp. Quotienting by H retains selected factors, so the finite-grid ideal is radical. This does not establish radicality or solving degree of the original unreduced selector chain. |
| Dynamic splitting | A proper gcd of a coefficient with squarefree q gives coprime child moduli; unit inversion then becomes valid. There are at most e terminal blocks. Restarting from original inputs after a split is correct but can duplicate work. |
| Empty selectors | F=1/G=1 give empty grids, distinct from H=0 on nonempty domains. No zero modulus is admitted. |
| Cancellation-coordinate lower bound | Under the note's full-selector, X-in-V, Y-nonzero hypotheses, every t fiber has the vertical point x=X and at least one S3 partner. They coincide only at the unique doubling coordinate. Thus the lower bound 2e-1 and the equality classification follow. |

No mathematical correction was needed for these statements at their stated
hypotheses. I suggested using log(2d) in the norm-cost formula so the written
bound is uniform at the admitted d=1 endpoint. The proof map should also
include the new cancellation lemma as a separately unformalized obligation.
[pcc:unverified]

## Cost attribution checked against primary sources

I independently opened the Dahan--Moreno Maza--Schost--Xie author preprint,
`https://cs.uwaterloo.ca/~eschost/publications/DMSWX-05b-TC-2006.pdf`.
Proposition 2.4 supplies the one-variable arithmetic-time bound; Proposition
4.1 multiplies it by the fast outer XGCD cost. Their composition supports
the stated fast half-gcd target in THEORY equation (12). Its hypotheses
include the paper's multiplication-time and organized splitting framework.
It does not certify this implementation's restarted schoolbook complexity.

I also opened van der Hoeven--Lecerf, *Directed evaluation*, section 1.1,
`https://www.texmacs.org/joris/direval/direval.html`. Its discussion reinforces
the explicit caution about recomputation overhead. The implementation must
not obtain a fast bound merely by naming dynamic evaluation.

The quotient norm has an explicit B-by-e coefficient state. Its logarithmic
number of ring operations does not make those operations constant-cost.
The code's `largest_tensor_slots` measures reduced multiplication outputs,
not transient allocations or peak resident memory. `ring_multiplications`
and `output_coefficients` are structural diagnostics, not a fully priced
operation or memory comparison. [pcc:unverified]

## Cancellation prediction and signed-output limits

The unique diagonal root comes from a linear polynomial with coefficient
-4Y^2, which is nonzero under the note's hypotheses. The proof includes that
root in the right support because X itself is a selector coordinate. For all
other right roots, the existence of at least one distinct selector partner
gives the extra point. Equality at 2e-1 forces exactly one x root above the
doubling coordinate and exactly two elsewhere.

For the historical d63/X1 fixture, 133=2*67-1 predicts aggregate degree mass
one at gcd degree one and 66 at gcd degree two. This constrains aggregate
modulus degrees, not the number of algorithmic blocks. Further valid
splitting can refine the degree-two part. The prediction uses a previously
recorded fixture; it is not an independent random-target test.

The coordinate mask is not a useful-relation filter. The example Q=3P is a
valid warning: the same projected pair can support P+P+P and a cancelling
triple. Recovering all eliminated coordinates and actual signs is necessary
before classifying signed rows. The native classifier also emits descriptive
masks for selector targets with Y=0; those cases do not satisfy the lower
bound lemma and must not inherit it. [pcc:unverified]

## Native-source invocation and mutation review

The producer invokes `verify_s3_input` on all 88 intended curve cases before
their records are emitted. It invokes `verify` and `verify_grid` on every
one of the 95 intended cases. The 27 predetermined persisted objects also
run certificate mutations. These are source observations, not completed
test counts. [pcc:unverified]

The certificate checker has six unconditional mutation classes and six
additional classes when the right partition is nonempty: 12 classes in that
case, six for G=1. These are distinct from the producer's 13 refusal controls:
eight invalid moduli, three invalid H tensors, one unsupported composite
field, and one singular curve. Actual executed counts must come from the
retained call output. [pcc:unverified]

Each proposed mutation necessarily violates a checked invariant. In
particular, changing the F-Bezout multiplier by one changes the identity by
the nonzero monic polynomial F; changing the F-cofactor by one changes it by
nonzero monic D. Missing or duplicated branches violate product/coprimality
coverage, including all-empty support. None of these mutation controls
depends on treating a rejected or capped query as empty.

`native/checker.rs` adds a persisted-object mode which invokes neither the
producer solver nor norm construction. It rereads the complete certificate
and input sidecar, checks the same independent identities/grid/input binding,
and additionally checks the supplied public target ordinate. With a raw
JSONL path, it binds certificate bytes to the recorded hashes, sizes, support,
dimensions and persisted case set. It explicitly records that sidecar bytes
were not separately hashed in the producer's raw record; their mathematical
input binding and freshly emitted hashes remain visible. This mode verifies
the persisted subset only, not the 68 omitted proof objects.

At the requested pre-freeze handoff, `native/independent.rs` has SHA256
`f727fc31a662eb3a7e7516eff674f57aa28beecf346a0eb94f7bb8bb0fadfe2c` and
`native/checker.rs` has SHA256
`290ce46f9a7d731452a46caeeaf3cf35af5125b71c68e14a754183e47586b0fd`.
No compiled input is edited by this review after that handoff.

## Remaining qualification

The coordinator still must build the frozen source, retain all observed
outputs and failures, and obtain a separate-role replay. The new written
statements still lack checked Lean declarations. The actual signed-output
contract still needs complete witnesses, lift/sign/subgroup checks, identity
branches and group replay. No performance comparison or signed scientific
validation is inferred from this review.
