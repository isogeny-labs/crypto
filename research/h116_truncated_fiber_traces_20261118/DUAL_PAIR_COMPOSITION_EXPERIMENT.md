# H116 dual-pair composition experiment

Frozen protocol `H116-DUAL-PAIR-COMPOSITION-001` passed all eight cells. The
independent verifier reproduced every exact correctness field, every locked
pole/multiplicity/regular-count prediction, and all three extension-stable
coefficient families. [pcc:unverified]

## Receipts

```text
916cff75615a65cd56e749a443213b50842c224957513c642f98679de888b1d2  DUAL_PAIR_COMPOSITION_PROTOCOL.json
4221e3f6ea1fe7fe5ad559b050d6bf8a76e8e5493f0f88421ae0ea6deae1b0ec  h116_dual_pair_composition.rs
b89f26b549eec491f29812129885b7ce57d2ddda5ad380bec21da5d1aeaab9d4  h116_dual_pair_composition_verify.py
b7ad7021167eb89a6b32cb0b1e8b4c5d704a075d67c2f32a9256d1628d1c3458  DUAL_PAIR_COMPOSITION_RESULT.tsv
5d5e3bb123c013c71992a3b89acd9bb80125c780ca51dd6bec491057d9264597  DUAL_PAIR_COMPOSITION_VERIFY.json
```

## Reconstructed right factor

The experiment reconstructed the x-map of `pi-1` jointly, without inserting
its known base-field kernel polynomial. Both the first and last canonical
`2N` regular windows had full rank and recovered the same coefficients in all
eight cells. Coefficients below are ascending; denominators are monic.

```text
p=5:  Npi(x)=[1,0,3,3,0,4,2,1]
      Dpi(x)=[0,0,4,1,2,2,1]
p=7:  Npi(x)=[2,6,2,4,3,1]
      Dpi(x)=[0,0,4,3,1]
p=11: Npi(x)=[4,7,1,0,4,9,7,4,8,0,10,6,6,9,1]
      Dpi(x)=[0,0,1,4,10,7,2,1,9,10,6,6,9,1]
```

The exact degrees are `(7,6)`, `(5,4)` and `(14,13)`, respectively—the same
degrees independently reconstructed for `V-1`. All 2,505 regular x-values
matched, all 29 pole x-values were found, and there were zero pole
cancellations. The denominator roots had the locked multiplicities: three
double roots for order 7, two double roots for order 5, and one simple plus six
double roots for order 14. The strict rational-separation margins ranged from
1 through 1,226.

## Complete composition census

For every point `P` in every frozen extension group, the producer and verifier
independently evaluated

```text
([t-1]-pi)((pi-1)(P)) = [N]P.
```

There were zero failures across all 5,066 points. This is the pointwise norm
factor in the same orientation as the Lean ring theorem
`(V-1)(pi-1)=[p+1-t]`.

Both preregistered algebraic mutations rejected in every cell:

- using `pi` instead of `pi-1` produced 5,040 failures;
- comparing with `[N+1]P` produced 5,058 failures, exactly every nonidentity
  point in the eight cells.

Adding one to the reconstructed numerator constant rejected all 2,505 regular
samples. Adding one to the denominator constant destroyed all 29 poles. The
`2N-1` boundary failed the strict separation premise in every cell.

This supplies a finite witness for the paired-factor degree pattern and norm
composition. It does not prove `(V-1)^vee=pi-1`, define scheme-theoretic
isogeny degree, establish the generic degree theorem or separability, solve
ECDLP, or establish subexponential runtime. [pcc:unverified]
