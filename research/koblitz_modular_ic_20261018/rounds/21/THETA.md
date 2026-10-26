# Equivariant integral recovery and its denominator

Use the exact G25 row map Phi:E^4->E^20, CM order Z[w] with w^2+w+2=0,
and G28's integral split recovery L*Phi=I. In this G25 basis the source
rotation is R(v)=(-sum(v),v0,v1,v2); each target5-cycle Pi satisfies
Pi*Phi=Phi*R. This basis differs from Q8's difference-coordinate descent.
The ordinary CM action is defined over F2 for each Koblitz sign.

Set A=sum_{k=0}^4 R^(-k)*L*Pi^k. The native coefficient computation checks
A*Phi=5I and A*Pi=R*A over the actual integral CM order. The three actual
matrix identities, including row equivariance, are kernel-checked Lean
statements. Thus A is an F2 descent-compatible morphism recovering [5]
on the source, under the written CM-morphism/descent bridge. No division
by5 is asserted to be an integral morphism.76 of its80 CM entries have a
coefficient not divisible by5.

Denominator5 is necessary for an equivariant rational left inverse in
this CM matrix category. Reduce modulo5. The source norm1+R+...+R^4 is
zero, but its fixed vector(1,1,1,1) is nonzero. On the four regular target
5-cycles every fixed vector is a norm: put that constant vector only in
one slot of each cycle. An equivariant recovery sends such a target norm
to the source norm, hence to zero. If g*f=dI with f and g equivariant,
apply this to the fixed source vector: d=0 modulo5. In particular d=1 is
impossible, and any cleared denominator is divisible by5. A/5 attains it.

Lean proves this structural obstruction over every nontrivial commutative
ring of characteristic5, exposing all linearity and equivariance
hypotheses. Applying it to the CM reduction (a field of25 elements, since
w^2+w+2 is irreducible modulo5) and reducing integral CM matrices is the
written bridge; the structural theorem does not require a field. This
also works without using the irreducibility observation.

The proper closed source immersion and descent in Round20 remain valid:
a closed immersion need not have an equivariant integral ambient
retraction. This obstruction concerns this regular-target CM presentation,
not all possible embeddings, all rational maps, or the final105/19 quotient.
It supplies no full theta action or rational divisor trivialization.
The243 final quotient sections, kernel action matrices and quotient
projective ideal remain open.
