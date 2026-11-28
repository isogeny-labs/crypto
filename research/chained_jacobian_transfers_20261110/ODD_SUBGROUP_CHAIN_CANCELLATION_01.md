# Cleared chain identities on stated odd subgroups

The additive module `lean/ClearedChainOddSubgroup01.lean` derives pointwise cancellation from an explicit annihilator and Bézout certificate. It applies the result to the existing two-hop outer norm/pullback identity, without requiring multiplication by4 to be injective on every ambient point. Both unchanged original Lean4.35/4.34.1 gates pass9 audited theorems/0 trusted: seven new named statements plus the two unchanged original ChainCompression statements. Full source, original outputs and exact PCC bindings are in `evidence/odd-subgroup-chain-cancellation-final-02`, dataa8584b. Claims remain unsigned. [pcc:unverified]

## Recover and cancel within the stated subgroup

Let A be an integer module. Supply integers d,r,u,v satisfying ud+vr=1. If [r]x=0, then

    [u][d]x = [ud]x = [ud+vr]x = x.

Consequently, [r]x=[r]y=0 and [d]x=[d]y imply x=y. `recover_annihilated_point`/e6ad94 and `cancel_on_annihilated_points`/869df9 prove these statements with the certificate and both annihilator hypotheses explicit. Ambient d-torsion is permitted. For d=4, the certificate covers any supplied odd exponent with an appropriate inverse; no primality assumption is needed. [pcc:unverified]

For zero-preserving integer-linear maps F,H from a source annihilated by r, linearity transports that source annihilator to both outputs. Thus [d]F=[d]H implies F=H on that source when the same certificate holds. `cancel_cleared_maps_on_source_exponent`/1e7919 proves this directly, without any injectivity assumption on the ambient target. Actual source subgroup and homomorphism properties remain inputs. [pcc:unverified]

## Apply the original two-hop identity

Retain every original hypothesis of `ChainCompression.two_hop_double_cleared`: the two cleared map identities, the middle norm/pullback relation and the stated integer linearity. Write

    G(x) = g(f(x)),       H(x) = k2(b(a(h0(x)))).

The original theorem gives [16]G(x)=[4]H(x). Put X=[4]G(x) and Y=H(x); then [4]X=[4]Y. The new theorem assumes [r]G(x)=0 and [r]H(x)=0, which also gives [r]X=0. A certificate 4u+vr=1 now cancels the last4 and gives

    [4]g(f(x)) = k2(b(a(h0(x)))).

`two_hop_factorization_on_annihilated_outputs`/725549 proves this exact conclusion. The theorem states the two output annihilators explicitly. The source-exponent theorem explains how actual homomorphisms from an r-annihilated source can supply them. The original geometric identities and cardinalities are separate obligations. [pcc:unverified]

The earlier written geometric argument cancels in the torsion-free Hom group of abelian varieties. Its geometric bridge remains separate. The new module instead formalizes a pointwise route on the specified annihilated outputs. An ambient2-torsion point is killed by4, so a global pointwise cancellation rule would require additional hypotheses. [pcc:unverified]

## Public quarter certificates

The three specializations are exact consequences of the same general theorem. Each requires its stated annihilator; the module does not construct or count a Jacobian. [pcc:unverified]

| Stated annihilator r | Public inverse u | Integer certificate | Theorem claim |
| ---: | ---: | --- | --- |
| 109 | 82 | 4·82−3·109=1 | 71025c [pcc:unverified] |
| 113 | 85 | 4·85−3·113=1 | 722066 [pcc:unverified] |
| 12317 | 9238 | 4·9238−3·12317=1 | 65df52 [pcc:unverified] |

The first two match the existing fixed rational subgroup quarters. The third supplies the whole12317-annihilated point quarter under that explicit hypothesis. These are public scalar operations; their use does not require knowing a target's discrete logarithm. The proof introduces no measured operation counts or timings. [pcc:unverified]

## Evidence and next step

The final source has SHA256`ffd54023b7b5f1b26880d26c9feba6ae4c0de0cbe8d86c9f5e03fd9bb17ba002`. The entire original ChainCompression source remains its byte-exact prefix. The unchanged original gates/auditors, actual commands, complete logs and seven normalized declaration bindings are retained. Only propext, Classical.choice and Quot.sound occur; no new axiom, sorry or trusted native proof is used. Local direct compilation also passes. [pcc:unverified]

The accompanying diagram/PDF shows the two hypotheses needed for pointwise cancellation. Dedicated CI pins the exact source and both original proof toolchains. Actual geometric maps, subgroup annihilator proofs, global Hom formalization and matched transfer/construction/solver/recovery costs remain separate. The next geometric bridge can instantiate these scoped lemmas without asserting global injectivity of point multiplication. [pcc:unverified]

Sources checked: original ChainCompression, ChainTransfer.injective_with_recovery and ReverseRecovery formal sources; CHAIN_COMPRESSION_PROOF.md and REVERSE_CHAIN_PROOF.md for the actual map notation and Hom/point distinction; original Lean IntModule scalar identities. Primary-review receiptd13f12 records exact source digests. KB MCP unavailable, wrapper status/exact/negative/theorem queries127; no novelty or absence inference. Original mathematical sources/protocols and all earlier negative results remain unchanged. [pcc:unverified]
