# RR9 B8 triangular coefficient elimination

Eliminating three chart variables reduced negative certificate generation from **342 ms to 159 ms** median wall time and the multiplier count from **20,672 to 2,691** on the same two B8 negatives. Both reduced identities replayed independently. The positive reconstructed the same seven coefficients and replayed on the original curve. These are local component diagnostics on three frozen fixtures. [pcc:unverified]

## Exact reduction

For residual `(r,s)`, write `A=a0+a1*x+a2*x^2+a3*x^3`, `V=v0+v1*x+v2*x^2+x^3`, and `H=prod(x-x_i)`. The coefficient equations come from

```text
(x^2+r*x+r^2+curve_a)*V^2 - (x-r)*A^2 - 2*s*A*V - H.
```

The leading coefficient is zero. The coefficient of `x^7` has the form `2*v2+f2(A)`. Substituting `v2=-f2(A)/2` makes the coefficient of `x^6` equal to `2*v1+f1(A)`. The two substitutions give the form `2*v0+f0(A)` at `x^5`. Over this odd field the pivots determine a unique triple `(v0,v1,v2)` for each `A`.

The remaining five equations define a four-variable ideal. Its solutions extend to full solutions by the three substitutions, and every full solution restricts to one of them. This holds over the field and its algebraic closure. An identity equal to one in the reduced equations therefore refutes the full system without expanding the certificate back into seven variables.

The independent C++ verifier builds the coefficient equations by univariate convolution over symbolic polynomials. It checks the pivot identities in order, confirms the three printed reconstruction polynomials, and checks every coefficient of each reduced certificate. On the positive it verifies all full equations and the original-curve point sum. Changing one multiplier by one causes the identity check to fail. [pcc:unverified]

## Matched local run

The two systems use the same Singular 4.4.1 executable, optional arithmetic module, coefficient presentation and graded reverse lexicographic order. Each cell ran three times on a shared local host. Each call was bounded to 30 s and 768 MiB of virtual address space. The table pools six negative observations per arm and reports median and range in milliseconds.

| System | Operation | Solver ms | Process wall ms |
| --- | --- | ---: | ---: |
| Seven variables | `std` basis | 21 (20–27) | 38.5 (34–42) |
| Four variables | `std` basis | 19.5 (18–24) | 34.5 (31–39) |
| Seven variables | `liftstd` certificate | 312 (296–337) | 342 (330–364) |
| Four variables | `liftstd` certificate | 139.5 (128–174) | 159 (146–192) |

Process wall includes startup, setup and certificate text output. The `std` wall ranges overlap. Exact certificate wall time was 2.15× lower by median. The reduced multipliers had maximum total degree 15, up from seven, and had 7.68× fewer terms. Reduced negative transcripts were about 59 kB, down from 487 kB. The decrease is in the proof support and runtime on these fixtures. Degree increases because each reconstructed variable is a polynomial in the four remaining coordinates.

All 36 calls completed. The independent replay checked 12 exact identities and 12 positive solutions. The other 12 calls were basis-only negative decisions. The positive chart was

```text
455835358 518652820 754716693 932383545 62510441 1884059825 1142049404
```

The previous same-fixture coefficient study measured the group-law comparator near 21 µs on the negatives. That comparator was not rerun here. The lift remains far above that reference on B8, so this round stopped at B8. [Prior reference](../../prime_field_followup_20261117/rr9/RESULT.md)

## Replay

Run `./verify_evidence.sh` with GCC and the prior verifier source present. It checks the saved reduced certificates and positive, including the mutation test, without Singular. For a fresh cohort, run `./run.sh /absolute/path/to/Singular /tmp/rr9-triangular` and `./summarize.sh /tmp/rr9-triangular`. Use an installed Singular 4.4.1 or configure its bundled libraries and optional arithmetic module before running. The scripts save each solver invocation, its output and its timing to the supplied output directory. [Protocol](PROTOCOL.md) · [Per-call measurements](measurements.tsv)

The initial Singular script used an unsupported multi-assignment declaration in its certificate reporter. The final cohort uses separate declarations. The first C++ replay inherited four-bit monomial exponents, which were too narrow for the reduced products. The final verifier uses eight-bit exponents and independently verifies the regenerated identities. No aborted record enters the measurements table. [pcc:unverified]

The protocol's manual link points to the wrong page number. The correct Singular manual pages define the standard-basis operation and the multiplier transformation used here. [std](https://www.singular.uni-kl.de/Manual/latest/sing_364.htm#SEC404) · [liftstd](https://www.singular.uni-kl.de/Manual/latest/sing_294.htm#SEC334)
