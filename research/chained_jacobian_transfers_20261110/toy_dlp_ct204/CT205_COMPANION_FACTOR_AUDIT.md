# CT205 — independently counted companion factor and executable native gate

2026-11-18; continuation of [crypto PR #4715](https://github.com/isogeny-labs/crypto/pull/4715) stacked on [#4697](https://github.com/isogeny-labs/crypto/pull/4697). **[pcc:unverified]**

## Public finite-field calculation performed

For the existing CT204 target `E_3/F_31: y²=x³+3` and genus-two cover
`C/F_31: v²=u⁶+3`, independently enumerate the source and its second
elliptic quotient using only finite-field integer arithmetic:

- `pi_1(u,v)=(u²,v)` targets `E_3`.
- The other involution is `(u,v)→(-u,-v)`; for `u≠0`,
  `pi_2(u,v)=(3/u²,3v/u³)` targets `E_9: y²=x³+9`.
  The maps extend at omitted affine/infinity points; the script checks
  all **48** rational affine points on which the explicit expressions apply.
- Direct point counts: `#E_3(F31)=43`, `#E_9(F31)=39`;
  their Frobenius traces are `-11` and `-7` respectively.
- Independent genus-two census: `#C(F31)=50`. The F31² enumeration,
  separately using `F31[t]/(t²-3)` with an explicit square set and
  `F31[t]/(t²-6)` with the quadratic norm character, gives
  `#C(F31²)=916` in both implementations.
- Therefore the genus-two Weil-polynomial coefficients are
  `a1=-18`, `a2=139`, and `#Jac(C)(F31)=1677=43·39`.
  These agree with the predicted isogeny decomposition
  `Jac(C) ~ E_3 × E_9` (degree a power of two).
- A separately coded affine group law enumerates all **43** distinct
  multiples of CT204's `G=(1,2)` and returns to infinity.

The companion has order 39, coprime to 43. **Every homomorphism from
the 43-order rational subgroup to the 39-order companion rational group
vanishes**. More strongly, these two elliptic curves have different
Frobenius traces, so they are not F31-isogenous and have no nonzero
F31-defined algebraic group homomorphism between them. Thus the degree-two
pullback transports the 43-order problem into the same 43-primary factor
of the larger Jacobian; it cannot shift that logarithm into the 39-order
companion. This is a **specific factor obstruction**, not a general
lower bound against all possible Jacobian algorithms.

The CT204 BSGS source analytically performs 670 elliptic additions and
670 *more expensive, distinct* Jacobian additions over 43 targets, plus
86 pullbacks and 43 pushforwards. Those are expected counts and are not
native timing evidence; no end-to-end speedup has been measured.

## Important pre-existing result and solver compatibility

This explicit sextic family, the complementary `E_{b²}` quotient and
the predicted genus-two IC scaling loss were already recorded in
[crypto-autoresearcher IDEA-20261109-4668b8](https://github.com/isogeny-labs/crypto-autoresearcher/blob/main/ledger/proposals/IDEA-20261109-4668b8.yaml).
This run only independently instantiates and checks the `p=31,b=3`
case; it claims **no novelty** for the decomposition.

[crypto PR #4676](https://github.com/isogeny-labs/crypto/pull/4676)
already implements a prime-field order-free IC solver on **odd-degree**
hyperelliptic models. CT204 instead uses the even-degree sextic with
two rational points at infinity. Its rational Weierstrass branch points
would require `u⁶=-3=28 mod 31`, but the sixth powers mod 31 are
`{0,1,2,4,8,16}`; there is **no rational branch point**. An
F31-isomorphism to an odd-degree hyperelliptic model would supply a rational
Weierstrass point, hence does not exist. The existing odd-degree solver
cannot simply be wired in by a coordinate change. The actionable
implementation work is an even-degree balanced-divisor IC engine or a
different transfer family with supported odd-degree destination. Do not
write a duplicate IC engine before reviewing PR #4676's existing collector.

## General parity obstruction for this odd-prime-order cover family

There is a short obstruction independent of the p=31 numerical example.
Let `p>3`, `b≠0`, and `E_b:y²=x³+b` have **odd full group
order** over Fp. Let `C_b:v²=u⁶+b`, with the natural degree-two
quotient to E_b. A rational Weierstrass point of C_b would have
`u₀⁶+b=0`; then `(u₀²,0)` is an Fp-rational point of order 2 on
E_b, contradicting odd order. The two infinity points of the even-degree
sextic are unramified, so they are not Weierstrass points. An
Fp-isomorphism to an odd-degree hyperelliptic model requires a rational
Weierstrass point. Therefore **no such odd-degree model exists over
Fp when the entire source elliptic group has odd order**.

This argument does **not** exclude using a large odd prime-order
subgroup of an *even*-order E_b; the total-group parity condition is
essential. The independent F31 program additionally checks every
nonzero b in F31, finding 20 odd-order cases and no forbidden rational
branch point. It is a finite consistency check, not the proof.

The resulting implementation boundary is concrete: to test CT204's
prime-order F31 instance with the previously implemented odd-degree
IC in crypto #4676, one must implement balanced even-degree Jacobian
relation handling (or choose a different family); a coordinate
reparametrization of C cannot bridge the gap.

## Reproduction

```sh
python3 research/chained_jacobian_transfers_20261110/toy_dlp_ct204/independent_quotient_oracle.py
cargo test --locked --release --test prime_cover_toy_complete_dlp -- --nocapture
```

The **first** command was locally reproduced under Python 3, with all
assertions passing. The second command remains an **attempted native CI
gate**, not a locally executed result. The bounded
`.github/workflows/ct205-toy-transfer.yml` has a small independent
Python job and a separate Rust 1.98 release-test job with a 40-minute
limit, two Cargo build workers, no S3 restoration and no EKS/Spot launch.
Inspect the exact-commit CI logs before changing the scientific standing.
Do not infer success from workflow creation.

## Next scientific decision

First resolve native toy arithmetic correctness and establish a genuine
Jacobian IC arm (including full setup/relation/LA/target-recovery cost),
or switch to the higher-value frozen direct-vs-retained transfer primitive
pricing once authenticated 695/701 fixture custody and model access
become available. Compare the fastest complete cold solver, not
Jacobian BSGS counts alone. No protected merge, paid experiment,
independent PCC verdict or measured speedup is claimed. [pcc:unverified]
