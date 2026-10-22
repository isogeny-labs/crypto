# Quartic twist Frobenius plus cubic automorphism: frozen probe

Frozen 22 October 2026 before implementing or running the native probe.
This is a new hypothesis prompted by reconciling the prior GLS study with
`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` E18. It is not an E18 cost result.

## Hypothesis and exact construction

For primes `p ≡ 1 mod 12`, let `d` be the least quadratic nonsquare in
`Fp`, let `F=Fp[t]/(t⁴-d)`, and let `E0/Fp: y²=x³+b0`.
The element `t` is nonsquare in `F`; set `E/F: y²=x³+b0 t³`.
Write `h=t^((1-p)/2)`, `z=t^(p-1)∈Fp` (order four), and
`ω∈Fp` of order three. On `E(F)` test

* `ψ(x,y)=(h² x^p,h³ y^p)`, the transported `p`-Frobenius;
* `ι(x,y)=(ωx,y)`, the ordinary `j=0` automorphism.

The predicted identities are `ψ⁴=[-1]`, `ι³=1`, and `ψι=ιψ`.
For `x∈Fp`, `ψ_x(x)=z⁻¹x∈Fp`, so the same line base used in E18 is
closed under both maps. If the full group has prime order `r`, their
eigenvalues on `G` should generate order 24, yielding 12 abscissae per
column versus E18's 3. This statement needs explicit point, scalar,
base and cost checks; the predicted asymptotic linear-algebra discount
is not a measured pipeline gain.

## Frozen candidate panel and controls

Use exactly the pairs `(p,b0)=(13,4),(37,2),(73,5),(97,5),
(157,25),(229,6),(241,49),(337,100)`, selected by an untimed
arithmetic screen for prime order. Recount `#E0(Fp)` by exhaustive
Legendre character sum and derive `#E(Fp⁴)` from the exact trace
recurrence and quadratic twist sign. Use deterministic 64-bit primality
checks. Verify `[r]P=O` for the generator and sample points. For `p=13`,
also independently enumerate all `Fp⁴` abscissae and square characters
to check the order without the recurrence. Retain any failed candidate.

For each curve enumerate all `x∈Fp` with liftable points on `E` and all
corresponding `±P`. Check curve membership, both maps, relations,
commutation, scalar eigenvalues and order on every base point. Partition
the physical base under `⟨-1,ι⟩` and `⟨ψ,ι⟩`; record orbit-size
histograms, numbers of unknown columns, and any fixed-point exceptions.
The same exact point set must feed both arms. An independent verifier
must reconstruct the field and group arithmetic and replay these claims.

## Next decision and claim boundary

If the 24-action and line closure verify, extend the existing E18
quartic relation stream and matched rho to this exact panel. Hold the
physical base, relation oracle, target, seed and stopping rule fixed;
measure full-rank rows, linear algebra, target descent, cold total,
and a rho walk folded by all 24 elements, with independent scalar replay.
The same curve identity and isomorphism must be documented before
comparing to historical E18 rows. If the action or closure fails, retain
the counterexample and do not benchmark a fictitious fold. This probe
alone cannot establish a new IC speedup, a lower crossover, or an attack
on a deployed curve.

## Registered follow-up after the structural probe passed

The native probe and independent Python verifier passed on the seven
nonempty-line candidates; `p=13` remains an empty-base control. Before
building the relation-pipeline extension, freeze the full-run panel:
`p∈{37,73,97,157,229,241,337}`, the exact `b0` values above, one
known-answer target `[271]G` per curve, and three independent public
seeds `0,1,2`. For each run, the **same** list of one signed point per
liftable `x∈Fp`, the same pair table, the same residual stream and the
same one-verified-decomposition-per-target choice feed three arms:
flat abscissa columns, cubic-automorphism columns, and combined
twist-Frobenius/cubic columns. Continue each arm until its target log is
independently verified after a full-rank square-core solve, or until
50,000,000 residuals have been tried. Keep failed, censored and
singular attempts. All group additions, field multiplications, row
operations, factor-base construction, table construction, collection,
linear algebra, online recovery and verification must be accounted for
before an end-to-end cost claim.

The matched rho controls use negation, the cubic automorphism and the
full 24-element action on that same target, with 16 independent walks
per curve and seed, each with its own table. The existing E18 r-adding
walk's look-ahead and fruitless-cycle doubling rules remain; charge
endomorphism canonicalisation. Report ratios per curve and growth with
size. A pilot on the three smallest positive curves may precede the
full panel but cannot replace it. If any arm or reference fails, retain
its raw record and withhold success-only speed ratios.

## Separate invariant-line sweep, registered before execution

The `p=13` empty line and the varying densities in the seven positive
rows show that `x∈Fp` need not be the best invariant factor base. The
`p`-Frobenius eigenspaces in `F_{p⁴}` are exactly the four lines
`x∈t^j Fp`, `j=0,1,2,3`; twist Frobenius acts on their abscissae by
`z^{j-1}` and the cubic automorphism by `ω`. For each of the eight
frozen `(p,b0)` pairs, enumerate every nonzero abscissa on all four
lines, test exact liftability, check both endomorphism images on every
lifted point, and report the line sizes and orbit histograms.
Use the same quadratic-character test for all lines and retain zero
lines. The maximum-size line per curve is a post hoc candidate; its
relation yield and complete cost require a separately frozen paired
run. Do not substitute it into the already registered `x∈Fp` panel.
