# Prime-degree multiplicity floor and composite-degree opening

16 November 2026. This note completes the class-number condition in
[`../ghs_metacyclic_general_factor_20261116/CLASS_GROUP_SCREEN.md`](../ghs_metacyclic_general_factor_20261116/CLASS_GROUP_SCREEN.md).
The result is a paper theorem for a specified family of covers and
ordinary elliptic factors. It does not classify all GHS covers or
establish an efficient discrete-logarithm attack. It is not yet
Lean-checked. Claim standing: [pcc:unverified].

## Exact scope and conclusion

Let `ell=6n+1` be prime, with `n>1` odd and `gcd(n,3)=1`.
Let `q` be a prime power of characteristic prime to `3*ell`
such that `ord_ell(q)=n`. Let `Q/F_q` be the order-three
quotient of a tame cyclic `ell`-cover with the branch and lift
hypotheses in the linked note. Then

`P_Q(X)=R(X^n),   deg R=2(3r-2)`.

**Theorem for prime `n`.** If `n` is prime and an ordinary elliptic quadratic
`g_t(Z)=Z^2-tZ+q^n` divides `R(Z)` with odd multiplicity,
its elliptic isogeny class over `F_(q^n)` is the base change of
an elliptic isogeny class over `F_q`. Equivalently, every ordinary
elliptic factor of `R` from an isogeny class absent over `F_q`
has even multiplicity. For two branch orbits, such a factor
therefore consumes at least four of the eight degrees of `R`,
or dimension `2n` in `J(Q)`. [pcc:unverified]

For composite `n`, the same conclusion holds whenever
`gcd(n,h(-ell))=1`. If the gcd exceeds one, this argument
leaves an arithmetic opening; it does not assert that an
elliptic factor occurs. [pcc:unverified]

The representation proof in the linked note shows that an
odd-multiplicity ordinary factor must have endomorphism field
`L0=Q(sqrt(-ell))`. Its class-group argument proves the theorem
when `gcd(n,h(-ell))=1`. The inequality below is the remaining
class-number input. [pcc:unverified]

`h(-ell) < n=(ell-1)/6`

holds throughout the family. It implies coprimality for prime
`n`, while composite `n` requires a separate gcd check.
[pcc:unverified]

## Uniform class-number bound

Write `p=ell`, let `chi` be the primitive quadratic character
of discriminant `-p`, and let `H_m` be the `m`th harmonic number.
The imaginary quadratic class-number formula gives

`h(-p) = sqrt(p)/pi * L(1,chi)`.

Because `sum_(a=1)^p chi(a)=0`, split the Dirichlet series into
the first `p-1` terms and complete later blocks:

`L(1,chi) = sum_(a=1)^(p-1) chi(a)/a`
`          + sum_(j>=1) sum_(a=1)^(p-1) chi(a)*(1/(jp+a)-1/(jp))`.

The first sum is at most `H_(p-1) < 1+log p`. The absolute
value of the later blocks is bounded by

`sum_(j>=1) sum_(a=1)^(p-1) a/(j^2*p^2)`
`  = (p-1)/(2p) * (pi^2/6) < pi^2/12 < 1`.

Thus `L(1,chi)<log p+2` and

`h(-p) < sqrt(p)/pi*(log p+2)`.

For real `x>=200`, the right side is less than `(x-1)/6`:
at `x=200`, `sqrt(200)*(log(200)+2)/pi < 32.90 < 199/6`,
and the difference `(x-1)/6 - sqrt(x)*(log x+2)/pi`
increases thereafter. Its derivative is
`1/6-(log x+4)/(2*pi*sqrt(x)) > 0` at 200, and the
subtracted term decreases for `x>=200`. Consequently
`h(-p)<(p-1)/6` for every prime `p>=200`.
[pcc:unverified]

The eligible primes below 200 and their exact class numbers are:

| `ell` | `n=(ell-1)/6` | `h(-ell)` |
| ---: | ---: | ---: |
| 31 | 5 | 3 |
| 43 | 7 | 1 |
| 67 | 11 | 1 |
| 79 | 13 | 5 |
| 103 | 17 | 5 |
| 139 | 23 | 3 |
| 151 | 25 | 7 |

Each entry has `h(-ell)<n`. These class numbers were read from
the [LMFDB number-field database](https://www.lmfdb.org/NumberField/)
via its `nf_fields` table, using exact labels
`2.0.ell.1`; all seven rows report `used_grh=false`.
The field `Q(sqrt(-31))` also admits the direct reduced-form
check `(1,1,8), (2,+/-1,4)`. The analytic formula and a
stronger general class-number estimate appear in
[Sutherland's thesis, Section 10.3](https://math.mit.edu/~drew/sutherland-phd.pdf).
This proves the claimed inequality in the stated family.
[pcc:unverified]

## Composite-degree exception and meaning for new-curve searches

For prime `n`, this closes the *simple ordinary factor from an
extension-only isogeny class* route under the metacyclic
hypotheses. It does not exclude repeated ordinary factors,
supersingular factors, other group actions, other quotient maps,
or transfers through higher-dimensional factors. The exact
`F47` six-branch cover in the linked note has `n=5`, so an
odd-multiplicity ordinary factor would be a subfield-class
rediscovery. A repeated non-subfield factor is its remaining
ordinary elliptic possibility. [pcc:unverified]

For composite `n`, the first class-group overlap in the LMFDB
query `ell<=100000`, sorted by `ell`, is `ell=463`, `n=77`,
`h(-463)=7`, class group `[7]`, `used_grh=false`.
Here `gcd(n,h)=7`. This is an **arithmetic exception to the
coprimality screen**, not an identified elliptic factor or
vulnerable curve. A six-branch quotient would have genus
`4n=308`, making full cost screening especially important.
The LMFDB query filtered imaginary quadratic fields with one
ramified prime, `ell=7 mod 12`, `gcd(n,3)=1`, and
`gcd(n,h(-ell))>1`; it returned this as the least row.
[pcc:unverified]

Any surviving factor still needs a constructive
subgroup-preserving map and a complete same-target cost
comparison. [pcc:unverified]

## Source and execution boundary

The group-algebra step uses
[Carocca--Lange--Rodriguez](https://arxiv.org/pdf/0906.4185),
the elliptic isogeny-class existence step uses
[Waterhouse](https://www.numdam.org/article/ASENS_1969_4_2_4_521_0.pdf),
and the class-number bound above is derived explicitly. The
checked papers did not state this exact combination; a targeted
search cannot establish publication priority. The GHS
classification comparison with
[Iijima--Momose--Chao](https://eprint.iacr.org/2013/487.pdf)
remains limited to their stated direct-cover and isogeny
conditions. [pcc:unverified]

The shared KB was checked at revision
`9141e386254340a9bdba3036f2bb706a`, last refreshed
14 November 2026. Searches covered metacyclic six-branch
factors, multiplicity, and GHS isogeny transfer. Sources
checked (KB): `source:d83cb619b3afdb7b9d999946161057c7`
(`crypto-autoresearcher:knowledge/leads/ecdlp/transfer/LEAD-TRF-weil-restriction-jacobian-class.yaml`)
identifies the abstract carrier route but gives no class-number
screen. The KB predates this proof. [pcc:unverified]

The shared `docs/session-logs/2026-11-16.md` remains reserved
by Conductor T-6720. This note records the completed derivation;
new factor-polynomial experiments remain deferred until the
session log can be updated. [pcc:unverified]
