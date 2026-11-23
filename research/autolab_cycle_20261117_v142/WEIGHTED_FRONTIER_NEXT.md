# Next hypothesis: weighted integer-fiber frontiers

Let each original coordinate lie in [0,M_i], with M_i<p, and let v be a
nonnegative integer vector with a positive entry. Assume a globally certified
field identity c(z+t*v)=c(z)+kappa*t. For every original integer tuple theta,
set b=min_i floor(theta_i/v_i), over positive v_i, and u=theta-b*v.
Then u is in the original box and some positive-direction coordinate has
u_i<v_i. These boundary tuples form disjoint integer translation fibers.
The first index satisfying u_i<v_i partitions the frontier without duplicates.
Its exact cardinality is

    N = product_i(M_i+1) - product_i max(M_i+1-v_i,0).

Zero direction entries preserve their full coordinate factor. An admissible
pivot obeys 0<=b<=min_i floor((M_i-u_i)/v_i), over positive entries; this
range lies in [0,p-1]. A nonzero kappa therefore selects at most one b,
namely -c(u)/kappa mod p, followed by the joint integer bound and every
original predicate. Zero slope retains the complete admissible fiber when
c(u)=0. The normalization proves uniqueness of every original tuple even
when several integer fibers lie in one finite-field orbit. No nonwrap or
primitive-gcd assumption is needed for correctness. Dividing an integer
direction by its gcd, when compatible with the certified field slope, can
reduce N and must be charged and checked separately.

Falsifiable experiment, not executed in C142: freeze n=2,3; p=3,5,7;
all M_i<p and v_i in [0,3] with at least one positive entry. Compare complete
frontier/fiber reconstruction against independent Cartesian brute force,
including v_i=0, gcd>1, v_i>=p, modular aliases and zero slopes. Admit at most
8192 candidates/output, 90 seconds, 4GiB and CPU0. A cap refusal is incomplete,
not empty. Measure actual chart counts and total costs before integration.

The critical open applicability test is whether a certified correction
direction has a cheap integer representative relative to the original domain
widths. The formula alone does not establish compression for actual NIST
instances or the GF(2^83) coupled system. For extension-field additive domains,
the separate exact stabilizer T={t: v_i*t in D_i for every i} measures
domain-preserving translation capacity; T={0} refutes that saving. Neither
route establishes novelty or the matching all-roots release gate.
