# All-pair arithmetic on the growing smooth family

For every t, Lean gives the actual family of reduced polynomial pairs for
F3, f=Xⁿ+1, g=12t+2 and n=24t+5 a cyclic additive group law. Every sum
agrees with an independently defined unique normalized Cantor output.
The actual pair (X,1) has exact order n and generates every family pair.
The earlier polynomial-defined cycle step is exactly addition by this
generator. Genus and order grow without bound. [pcc:unverified]

All family pairs are smooth. Their smoothness proportion is 1, while the
full finite universe of valid reduced pairs has proportion strictly
smaller than 1, witnessed by (X²+X+2,X). The new result supplies all-pair
arithmetic and a checked group law **on the family**. A group law on the
entire ambient universe and its geometric Picard/Jacobian identification
remain separate formal obligations. [pcc:unverified]

## Arithmetic before group transport

1. The frozen parent defines all valid reduced pairs, proves ambient
   finiteness, actual polynomial separability and degree, n distinct split
   pairs and a uniform nonsplit pair. [pcc:unverified]
2. Same-sign inputs have monic gcd 1 and composition degree a+b. When
   a+b>g, exactly one reduction replaces (Xᵃ⁺ᵇ,s) by
   (Xⁿ⁻ᵃ⁻ᵇ,−s). Explicit polynomial Bézout, quotient and remainder
   certificates work for both signs s=±1. [pcc:unverified]
3. Opposite-sign inputs have monic gcd Xᵐⁱⁿ⁽ᵃ,ᵇ⁾. Unequal degrees leave
   the larger degree's sign and degree |a−b|; equal degrees cancel to
   identity. Identity and input-swap cases are certified too.
   [pcc:unverified]
4. The signed residue map is a bijection from actual family indices to
   ZMod n. The candidate output's residue equals the input residue sum.
   Every candidate has a full normalized run, and output uniqueness is
   proved with fixed explicitly selected Bézout coefficients.
   [pcc:unverified]
5. `pairwiseOutput` and `cantorAdd` are defined from those actual unique
   outputs before introducing the additive group. Only then is the group
   transported through the residue bijection. Its sum is proved equal to
   `cantorAdd` for every pair. This establishes the algebraic laws for the
   certified arithmetic. [pcc:unverified]
6. The generator has actual coordinates (X,1), exact order n, and every
   element is j times the generator for some j<n. Explicit equality of
   selected coefficients binds generator addition to the frozen parent's
   independently defined `actualStep`. [pcc:unverified]

| Module / namespace | Checked declarations |
|:--|:--|
| `AllPairRuns`, `MumfordAllPairRunsAudit` | `run_swap`, `identity_left_run`, `same_sign_run`, `opposite_strict_run`, `opposite_cancel_run` |
| `IndexArithmetic`, `MumfordIndexArithmeticAudit` | `residue_bijective`, `sameIndex_residue`, `differenceIndex_residue`, `combinedIndex_residue` |
| `PairwiseOutput`, `MumfordPairwiseOutputAudit` | `selected_run_candidate`, `selected_output_exists_unique`, `pairwiseOutput_runs`, `pairwiseOutput_eq` |
| `VerifiedFamilyGroup`, `MumfordVerifiedFamilyGroupAudit` | `scalar_cantorAdd`, `cantorAdd_eq_add`, `actual_sum_runs`, `generator_order`, `all_elements_generated` |
| `GeneratorAgreement`, `MumfordGeneratorAgreementAudit` | `selectedTriple_generator`, `generator_add_run`, `actualStep_eq_generator_add`, `growing_verified_group_certificate` |

## Source scope and assurance

The source review reuses the exact primary reads from the
[parent study](../mumford_family_audit_20261110/evidence/source-review.json):
Stoll 2014, Lemmas 4.16–4.17, Theorem 4.18 and Example 4.19, printed
pages 23–26; Jędrzejak 2016, relevant introduction/theorem, model,
good-reduction and printed-page-111 torsion proof. The family is known;
no novelty claim. Cantor's 1987 original remains unread after ordinary
AMS HTTP403. KB/Conductor unavailable; bounded primary/repository/Mathlib
fallback recorded. No new claim of a full-paper read. [pcc:unverified]

The conclusion does not assume a Picard correspondence and does not
prove one. It does not refute fixed-genus q→∞ or typical/large-prime
subgroup asymptotics. The orders 24t+5 are not all prime. The earlier
complete finite Stoll group remains a separate proper prime-order-five
subgroup counterexample at its stated reduced-pair-model scope.
[pcc:unverified]

Lean 4.35.0 and Mathlib 56a005855ea17e0b0958f73ec801ae140b0a1679 are
pinned. Only propext, Classical.choice and Quot.sound are admitted. No
holes, custom axioms, native_decide, linter waivers or added elaboration
settings. Issued parent sources remain unchanged. All rejected attempts
are retained losslessly. No new native experiment, sampling, timing or
paid compute. [pcc:unverified]

The copied prior IDC1hb7102c15384f2648 covers only its base-quintic scalar
identity. It does not certify this field, variable exponent, group law,
period, counting or geometry. Those new arithmetic conclusions rest on
Lean. [pcc:unverified]

```sh
cd research/mumford_family_group_audit_20261110/lean
lake build
cd ../../..
tools/lean/proof-gate.sh research/mumford_family_group_audit_20261110/lean
```

Source pins, full gate, preissue source commit, packet and actual verdict
are in sources.json and verification.json. See the
[report](reports/proof-20261110/report.pdf) and
[editable proof diagram](reports/proof-20261110/proof.dot). Signed standing
remains unverified. Existing IC performance, benchmark and corpus records
retain their prior scope. [pcc:unverified]

The full project gate passes 64 declarations, zero failures and zero
trusted-native exceptions. All 22 attempt/source files and five gate files
have byte-equal archive roundtrip checks. [pcc:unverified]

Source commit `1eb963ad` precedes `PCC-20261110-e41dcb`. Full local
verification passes at 13:28:02Z; signed standing remains unverified.
The final inspected report is two pages, 51476 bytes. [pcc:unverified]
