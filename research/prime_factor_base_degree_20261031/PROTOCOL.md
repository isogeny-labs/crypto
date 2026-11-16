# Prime-field degree pilot, preregistration v1

Status: pending execution. Objective: establish a genuinely novel useful
factor-base degree reduction, retaining a strict distinction among homogeneous
regularity, bounded-Macaulay refutation and actual iterative solving degree.
The current dense exporter is a finite control. Its homogeneous metric is fixed
by THEORY.md, so it cannot be promoted as a homogeneous regularity win.

## Candidate and controls

Freeze E/Fp, a checked prime-order full curve group, d|(p−1), nonzero c1,c2
and h≠0. Candidate x values satisfy `x^d=c1`, `(x+h)^d=c2` and have rational
curve lifts. Include both signs. Controls are the first B admitted x values
and B admitted x values selected by domain-separated hash order, on the same
curve. All three have the same cardinality B. Duplicate sets are reported.
No target outcome enters base construction or selection.

Initial bounded control spec is pilot.json: p=17, A=B=2, d=8, h=2,
c1=c2=1, seed315. This is a unit-fixture-derived smoke control, not a novelty
or significance test. The exporter enumerates the complete curve and requires
its order to be prime. It exports every target x, one representative of each
± target pair, and full exact point-sum existence references. It refuses
singular curves, composite field/group orders, unsupported sizes and overwrite.

No cryptographic-size run is proposed. Register and verify the exact ICV1/EC1
representation and canonical FB1 dumps before any result is promoted. The
development manifest labels its identity linkage incomplete instead of
inventing an accepted ID.

## Exact input and certificate

Dense input is `F_V(x),F_V(y),S3(x,y,r)` over ordinary Fp[x,y]. All roots of
membership are already rational lifts; twist-only membership is excluded.
Do not add Boolean equations or replace powers by squarefree masks.

For each inconsistent system, retain an upper identity
`1=Σ a_i m_i f_i`, with `deg(m_i)+deg(f_i)≤D`, and a lower dual vector
annihilating every such row at D−1 while pairing to one with the constant.
The native checker independently multiplies and sums the named polynomials.
It binds exact input bytes by SHA-256 and recomputes profile metadata.
No-refutation-through-cap stays unresolved; it never becomes an F4 degree.

Compute full input root sets and compare with group-sum existence. Replay
every relation, including doubles. Report SAT, UNSAT, UNKNOWN, reference
disagreements, rows/columns/ranks and certificate hashes on the complete target
population. Any mismatch stops promotion and remains in the raw records.

## Structured arm still required

Encode both shifted-power constraints by complete low-degree chains. Include
all rational-lift conditions, auxiliaries, denominator exceptions and branches.
Use the same candidate point set as the dense arm. Add a same-point-set
encoding control before crediting the factor base itself with a difference.
The shifted-chain exporter and an iterative F4 trace are not implemented here.
They must be implemented and cross-checked before this arm is launched.

For a positive degree claim, freeze the full variable/equation ordering and
invariant. Require a strict decrease of at least one on the identical complete
paired target population, zero false positives/negatives and nonzero natural
relation yield. Separate common-UNSAT from SAT populations. Lowering input degree,
fixing variables, losing branches or changing cardinality is insufficient.
Also report width, all native operations and peak memory. Lower degree with
higher total work does not establish an end-to-end improvement.

Only after a positive complete control: prospectively freeze at least two more
ordinary, nonexceptional prime-field curves, disjoint target/parameter holdouts,
and a growing-size ladder. A novelty claim requires independent full-text
prior-art review and a structural explanation surviving same-shape controls.

## Launch and limits

First control: one local process, one CPU, hard 2 GiB address-space and
120-second outer wall cap, only after either the existing S3/EKS admission
passes or the owner explicitly overrides it for this bounded local pilot.
An override must name both the local backend and failed checkpoint gate;
generic instructions to continue are not silently treated as one.
Native matrix limits are 4,096 rows, degree24, p≤65521 for certificate input;
the exact group exporter is limited to p≤257 and base size2..12.
Preserve every partial output and error. No paid job or resource expansion is
authorized by this document.

```sh
cargo build --release --locked --manifest-path research/prime_factor_base_degree_20261031/native/Cargo.toml
# After launch admission, in a fresh directory:
prime-degree-certificate prepare research/prime_factor_base_degree_20261031/pilot.json NEW-DIRECTORY
prime-degree-certificate certify INPUT.json NEW-CERTIFICATE.json
prime-degree-certificate verify INPUT.json NEW-CERTIFICATE.json
```

The complete screen driver must enforce the outer cap across all cells and
archive raw outputs with hashes. The protocol is frozen before any run; its
preparation and unit tests are not experiment execution.
