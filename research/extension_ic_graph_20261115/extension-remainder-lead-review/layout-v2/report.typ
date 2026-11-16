#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:9.4pt)
= Quadratic remainder classes as graph vertices
16 November 2026 · Static source review · Unselected next lead

The current two-large-prime extractor removes small Fp roots, requires the
remaining quadratic to split in Fp, then verifies a full Jacobian relation.
The original full-smooth path also requires all roots in Fp. This review
records an extension-field branch to investigate after the frozen trial39
comparison; it includes no new implementation or measurement.
[pcc:unverified]

#image("flow.svg",width:100%)

The actual line conventions are $A=a_0+t s a_1$, $C=sqrt(s)B$,
$y=-A/C$ and $F=A^2-C^2 H$. For a monic quadratic $U$ dividing $F$ and
coprime to $C$, the proposed remainder uses $V=-A/C mod U$ and must satisfy
$V^2=H mod U$. A principal-divisor argument and independent Jacobian controls
must justify the seven signed small-point classes plus the degree-two
remainder class. No conjugate-class identification is assumed.
[pcc:unverified]

Two partial rows carrying the same checked canonical remainder class with
coefficients $c_a,c_b$ could combine as $c_b R_a-c_a R_b$, eliminating that
unknown. The combined small-point row still needs its original-input witness,
rank selection, protected target/RHS and actual elliptic verification.
A remainder class may be composite; this diagram makes no prime-divisor
assertion. [pcc:unverified]

Competing hypotheses: the new partials and cheap certificates reduce relation
work, or a larger vertex universe, scarce collisions and verification/filter
cost remove that benefit. More partials alone establish neither target
recovery nor a pipeline improvement. Any future comparison must retain every
failure and charge the selected verification method. [pcc:unverified]

Actual source also gives a finite B-space of ceil(p/2) for m8, versus p²
for m9. Lower degree cannot be assumed to provide enough independent rows.
No m8 or remainder experiment is selected here. [pcc:unverified]

Sources: jv_sieve.rs (SieveSetup, b_space, relation_from_hit,
relation_from_roots), jv_sieve_two_large_prime.rs (low_degree_roots,
verified_relation), and Joux–Vitse sections3.2/3.3. Exact/negative KB
queries were unavailable and their receipts are retained. The companion
LEAD_EXTENSION_QUADRATIC_REMAINDERS.md lists the proof obligations and
future observables. No novelty, asymptotic, timing or rho-parity claim.
[pcc:unverified]
