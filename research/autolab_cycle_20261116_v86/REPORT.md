# C86: exact twist transfer and concrete auxiliary curves

The previous RAM workspace and terminal disappeared after an approval-service usage failure. Persistent older checkout/ledger were preserved unchanged. Public C84 head ff78a96e67f8b175ecfe66fee1bd760c1f53d6e7 is intact. C85's final registration/run command was explicitly not executed; its draft tests have no outcomes. This recovery starts a separate continuation ledger with predecessor hashes and public-head receipt. The vanished intermediate RAM journal is not claimed recovered. New tests are self-contained, versioned and frozen under fresh IDs.

Hypothesis1: twists may permit an exact two-curve representation of rational/anti-rational root components. Exhaustive tests passed35164S3 triples and124coordinate-type checks over12configurations, including binary even extensions,prime characteristic3,GF9 andGF27. Binary A0/A1 twists share the identical S3 polynomial for fixedB; odd twists give a coordinate-scaled equation. Twisting alone supplies no smaller polynomial system. Distinct curve labels cannot inflate a hard benchmark with identical polynomial/domain inputs. Torsion,zero and repeated coordinates remain covered.

Hypothesis2: auxiliary isogenies may compress factor-base domains, if all inverse fibers are restored. Explicit2isogenies passed160point-membership checks,2408complete group-pair homomorphism checks and complete reconstruction of1881finite-source point triples across14binary/odd configurations.608nonkernel binary S3 roots mapped forward correctly. An actual false converse is retained: GF8,A0,B1,source x=(2,2,6) maps to(7,7,5); auxiliaryS3 vanishes but original residual is2. Pole/kernel branches were retained and counted.

For current GF83 prefix D18 and B1, the2isogeny x-map x+1/x is injective on nonzero source coordinates: a collision requires xy1, and degree(xy)<=34<83 forces equality inF2[T], whose only nonzero unit is1. A second quotient step creates no new collision outside kernel x0,1 by the degree<=68 argument in PROOF.md. Derived Koblitz group orders have2adic valuations2(A0)and1(A1), limiting rational2power kernels. Thus degree alone supplies no x-domain compression on this input; orbit-closed factor bases would be a separately specified domain experiment.

Hypothesis3: odd-degree kernels on NIST quadratic twists could yield concrete auxiliary curves. All five field primes and generator subgroup orders were proved using FLINT fmpz_is_prime. Generator membership,N*G=O and Hasse uniqueness prove the original curve order; the quadratic-twist formula supplies its order. Results:

|Curve|Twist rational3kernel|Twist order3-adic valuation|Auxiliary outcome|
|---|---|---:|---|
|P-192|no|0|absent by proved order|
|P-224|yes|2|constructed and checked|
|P-256|yes|1|constructed and checked|
|P-384|no|0|absent by proved order|
|P-521|no|0|absent by proved order|

P224 andP256 have explicit exact-order3kernel points, verified independently by the3division polynomial. Their Velu target curves are nonsingular;13sample points and169homomorphism pairs per actual curve passed. Four complete small3isogeny cases reconstructed1170point relations, including every kernel fiber. Two small no-kernel cases were retained. This constructs and checks actual-width auxiliary curves, not a completed root search. The FLINT binary hash and source-identified presets are frozen; the deterministic proof interface emits no standalone Pocklington certificate.

The original NIST rational groups have prime order and no rational3kernel. Twist-assisted3isogenies therefore do not provide free fiber compression of original rational points. They may apply to genuine twist components after exact type/torsion routing or to deliberately redesigned factor bases. NIST connected finite chains with the established no2torsion guard already force rational components, so twist work cannot be presumed to accelerate those original instances.

Three experiments completed under90s/4096MiB/one-core limits; worker wall/CPU/RSS and every raw outcome are preserved. No runtime ratio,novelty clearance,full original-root solution or20instance release claim is made. Primary formulas are established twist/isogeny theory. The50xstretch goal remains active. Next queue retains both certified residue-table search and auxiliary/twist routes; neither replaces the other.
