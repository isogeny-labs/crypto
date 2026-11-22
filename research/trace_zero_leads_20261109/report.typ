#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
= Trace-zero leads: restrict and certify before solving
2026-11-09 · Isogeny-labs/crypto · PR3744 · local unsigned evidence

*Result:* Two concrete specializations are implemented and checked. A finite
admitted domain can replace redundant full-point membership witnesses. On the
actual small polynomial window, restricting q(x)=Tr(xH(x)) before coordinate
elimination preserves quadratic degree. Two further lead cards specify
complete quadratic relation chains and rank-aware presentation choice.

#image("leads.svg", width: 100%)

== TZL-01: finite-domain membership certificate
Every allowed x is taken from an original point admitted to the full group-trace
kernel Tₙ. Its negation is also in Tₙ. The added ordinate witnesses occur only
inside each summand's membership model, so every allowed x has a satisfying
choice independently of the S3 constraints. Existentially removing those
witnesses preserves the projected system. Actual supplied base signs still
matter and are checked during lifting and original-group replay.

The new API decompose_full_trace_zero_with_encoding accepts CertifiedDomain
as an opt-in alternative. Default ExplicitPoints is retained. Point admission,
finite x domains, identity patterns, original-group acceptance and conservative
resource caps remain. This specialization does not apply to an arbitrary
unadmitted algebraic subspace. Counts above concern the same singleton-domain
instance, before full-point pins; they are not timing evidence.

#pagebreak()
== TZL-02: the restricted second bit can be linear
For odd n, the half-trace identity gives q(x)=Tr(x)+St(x), where St is the
second elementary symmetric polynomial of all Frobenius conjugates of x.
The H term at exponent0 gives Tr(x²)=Tr(x). Its remaining even exponents
choose one from each nonzero difference class modulo n up to sign, so their
traces sum every unordered conjugate pair once.

Polarization therefore gives Bq(x,z)=Tr(xz)+Tr(x)Tr(z). On V inside the
field-trace kernel, Bq is simply the trace pairing. If its restriction is
zero, q on V is linear, but need not vanish. Existing production halving
then makes full affine membership Tr(y)=q(x).

The actual ECC2K-130 window V=span(z,…,z¹⁶) has q restricted identically zero.
It is the same retained subspace from the original span(1,z,…,z¹⁶) after
its x-trace coordinate is removed. No new factor base is selected.
Its product-span dimension is31. Restricting before eliminating y₀ gives:

#table(columns: (auto,auto,auto,auto,auto,auto), inset: 5pt,
 [n131,d16], [Variables], [Rows], [Terms], [Degree], [AND aux],
 [Raw], [147], [132], [3108], [2], [2216],
 [Compact], [146], [131], [3076], [2], [2200],
 [Factored], [147], [132], [3094], [2], [2216],
)

All three rows describe the same allowed affine points. Boolean variables
are implicit, with a separate identity branch. These are exact finite-point
models, not general geometric defining ideals. The earlier unrestricted
260-variable model describes another point set and is not this comparison's
baseline.

#table(columns: (auto,auto,auto,auto,auto,auto), inset: 5pt,
 [n], [d], [Polar rank], [Linear q terms], [Quadratic q terms], [dim(V·V)],
 [83], [16], [0], [0], [0], [31],
 [83], [32], [0], [1], [0], [63],
 [83], [64], [48], [4], [56], [83],
 [131], [16], [0], [0], [0], [31],
 [131], [32], [0], [0], [0], [63],
 [131], [64], [0], [1], [0], [127],
)

These are constructed profiles, one exact basis per row. At n131,d64 the
remaining q bit is the z⁵⁹ coordinate, so a zero-polar-form assertion must
not be promoted to q=0. At n83,d64 compact becomes cubic and has10646 AND
auxiliaries against7328 for raw/factored. This counter-control is retained.

Integral certificate IDC1h3f66989fcfd56337 records the polarization expansion
of S2 in16 formal coordinates, degree2,32 issued plus32 local replay points
over2⁶¹−1. It concerns that integral identity only, not the characteristic-two
descent or a formal theorem. The written expansion and production-source
native checks carry the field-model argument; independent standing is unverified.

#pagebreak()
== New deciding steps
*TZL-01:* compare explicit/certified modes on identical non-singleton bases,
unseen known-answer targets, seeds, all identity patterns and total budgets.
Charge admission, rejected models and successful replay. Reduced allocation
could also weaken propagation; only complete successful work settles it.

*TZL-02:* instantiate matched relation systems using restricted raw, compact
and factored point models. Record actual solving degree and complete work
on the same points and targets. Input degree alone is not solving degree.

*TZL-03, proposed:* retain full points and slope witnesses to obtain quadratic
addition constraints instead of eliminating y into S3. Include generic
addition with nonzero guards, doubling, opposite points and identity branches.
Do not constrain partial sums to the original V. First compare every small-field
case with the actual group. At m3,d16,n131 the summand models alone have438
variables, against179 in the naive x-only chain layout. The extra-variable
tradeoff must be charged; no complete builder or solver result exists yet.
S3 also admits quadratic lifting, already used by the native AND/XOR circuit.
Compare that baseline too; displayed input degree alone supplies no advantage.

*TZL-04, proposed:* use q polar rank, product-space dimension and actual term/
auxiliary counts to select a presentation while keeping V fixed. Test held-out
successful work against every fixed mode and a charged portfolio. Profile
statistics are not a solver-cost theorem; changing the subspace is a separate
controlled experiment.

== Validation and current limits
Seven new focused tests pass, including exhaustive restricted Boolean models
at n3/5/7, exact relations with sign-restricted and multiple-x finite bases,
malformed/cap rejection, independent small-field and public n83/131 subtrace
checks, and certificate replay/tamper rejection. The old ten-test full-variety
suite also passes. Both harnesses use actual production modules without
arithmetic replacements. Root cargo check --lib --no-default-features passes.

Certified singleton-domain identity controls are SAT at n83 and131 with415
and655 SAT variables, no ordinate pins, and original G+(-G)+G=G replay.
The original explicit n83 control still returns UNKNOWN at the same one-second/
2000-conflict cap. These are correctness controls, not natural-target relations
or a general performance comparison. The earlier whole-root failures and
benchrec limitation remain on the PR. No EKS or paid job ran; checkpoint/S3
access is still missing for matched experiments.

Flow: prior-art review, two implementations, exact constructions and local
checks completed; TZL-03/04 proposed; natural relation-finding, performance
and independent signed validation not attempted. Existing scoreboard and
performance graph values remain unchanged.

== Sources and reproduction
README.md gives locked commands and proof scope. LEADS.json maps all four
cards to existing atlas families. models.json preserves exact bases, restricted
ANFs and circuit counts; checks/ retains raw output and environment. Source
hashes pin this revision; previous studies retain their historical identities.

Sources: current crypto-autoresearcher ECDLP atlas, decomposition-encoding-and-sat,
factor-base-subspace-choice, cofactor-torsion-quotients and prime-degree-algebraic-
decomposition rows; Kosters–Yeo1503.08001; Trimoska–Ionica–Dequen AFRICACRYPT2020;
Ri–Myong–Kim–Rim1304.0521, definitions/Lemma3; existing production halving and
addition derivations. KB unavailable (wrapper127); PMC follow-up recaptcha.
No global novelty or new formally checked mathematical theorem is asserted.
