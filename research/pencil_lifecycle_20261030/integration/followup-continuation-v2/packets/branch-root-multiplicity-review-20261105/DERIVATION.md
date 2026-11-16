# Independent preread derivation

At a simple branch point, write f=(x-a)g with g(a) nonzero. In the actual local ring y²=(x-a)g, g is a unit, and y has valuation 1. Thus v(x-a)=2.

For a nonzero polynomial p, mathlib supplies p=(X-C a)^m q with m=rootMultiplicity(a,p) and (X-C a) not dividing q. The factor theorem gives q(a) nonzero. The actual affine branch evaluation maps p(x) to p(a); its kernel defines the branch prime. Consequently q(x) belongs to the localization denominators and its image through the projective stalk equivalence is a unit. Hence v(E(p))=2m+0, finite, and the corresponding Nat order is 2m.

The generic statement MUST exclude p=0: mathlib defines rootMultiplicity(a,0)=0, but v(E(0)) is infinity. A theorem only about toNat can conceal this because infinity.toNat=0; require the finite-valued equality before conversion. A nonzero constant has m=0 and unit image. Powers (X-C a)^m test arbitrary multiplicity, not merely the special 0/1 support case.

For the reviewed norm specialization, the three stalk support valuations sum to 2. Each factor must therefore have nonzero stalk image and be a nonzero polynomial. Substitution of the generic transport gives 2(m1+m2+mU)=2 and so m1+m2+mU=1. This is the base multiplicity sum, while the actual stalk sum remains 2. It does not double a Mumford base multiplicity.

Simple-branch and actual-map premises are essential. Characteristic two need not invalidate these local valuation identities, but separable double-cover or distinct-conjugate interpretations require separate hypotheses. The kernel and localization argument must prove residual-unit status instead of postulating it. No successor source, compiler, or runtime trial was consulted in this derivation.
