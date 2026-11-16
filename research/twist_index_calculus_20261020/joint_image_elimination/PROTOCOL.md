# Joint image elimination: algebra and complete-root protocol v1

Frozen before executing this round. Date: 2026-10-23. Parent evidence:
`endomorphism_image_base`, commit e84a0a05214d7fda5b912eea08029e43d5fd409f.
The earlier 189+21 cases, cold measurements and archive remain historical
evidence. Their timed source is not replaced by this round.

## Question and algorithm

Can retaining the variable image point in a joint relation system reduce
verified relation-finding work relative to the completed separate-peel S4
solver? Compare phi=1-psi with generic doubling on the identical declared
subgroup and same-size known-log images. Pure L0 is a further control.

The checked 49-term S4 expression H(e1,e2,e3,X;B) already uses elementary
symmetric variables for three L0 abscissae. This round constructs

    Res_X(H(e1,e2,e3,X;B), S3(X,z,x(R);B)),

then substitutes z=N(u)/D(u), clearing the eighth power of D. Phi uses
N=-omega^2*(u^3+4B), D=3u^2; doubling uses N=u^4-8Bu,
D=4(u^3+B); pure L0 uses N=u, D=1. This is a joint S5 construction.
It is not 2L separate S4 solves relabelled as one solve. The same generic
resultant construction and coefficient order apply to all three maps.

Two explicit encodings are retained. The geometric encoding saturates the
cubic discriminant by a fifth variable, excluding repeated ordinary
abscissae. Its denominator-zero and non-factor-base roots are retained as
rejected candidates. The finite-domain encoding requires the monic cubic
to divide the squarefree polynomial of allowed L0 abscissae and constrains
u to an allowed source abscissa. For pure L0, the product of that cubic
with (T-u) must divide the same domain polynomial; this excludes the fourth
factor repeating an ordinary factor. Neither encoding changes the required
physical signed relation set.

R=O uses H with its fourth coordinate N/D, with D^4 clearing; it is not
silently omitted. The original subgroup/base hypotheses remain checked.
Every accepted relation must have four distinct physical factor IDs and
sum to the supplied residual under the actual group law.

## Inputs and fixed construction suite

Use all 21 original records over p=37,73,97,157,229,241,337 and seeds 0,1,2,
with frozen tape SHA-256
d50c500961344a1d5813a02dbba9019aef3309cb74180e99715ca693f5b8094c.
Their scalar 271 and conditioned historical trajectories remain explicit
fixture limitations. This round does not provide independent target-scalar
sampling or measure replacing the curve by a different twist.

A preparation command emits an allowlisted public JSON file for each case.
It retains curve/order/action parameters, G,Q, public point encodings and
only the two public coefficients of each query. Known scalar, reference
hits, ranks, factor logs and diagnostic trace fields are excluded. The
candidate checks the public file's hash and exact key set before geometry.
The reference/audit modes are separate from the candidate mode; this is
an input separation, not a formal noninterference proof.

For every case and all three arms, construction inputs are the distinct
indices 0, floor(m/3), floor(2m/3), m-1, with m the frozen stream length.
The first independently recorded positive query is an additional labelled
correctness control when absent from that set. It is selected from the
already completed scalar/group reference, never from solver timings.
No timing-population inference is made from these conditioned controls.

Additionally, the numerical resultant must vanish for every complete
signed relation in the entire original reference streams of all 21 cases
and all three arms. Validate physical sums and map coordinates separately.
Compare the symbolic determinant with independent finite-field Gaussian
determinants at deterministic evaluation points. Retain coefficient/input
mutations and unknown-key rejection as negative controls.

## Root solving and cost gates

After construction checks pass, run native F4 and official msolve on the
identical exported systems, including geometric and finite-domain forms.
Native pair degree cap is 64 for every arm; record truncation, timeout,
allocation failure and incomplete root extraction explicitly. Independently
reduce inputs and S-pairs for every purported complete reduced basis, and
compare complete signed physical sets with the existing group oracle.
An incomplete solve cannot be called complete because it found a relation.

Keep one Rayon/solver thread, 8 GiB address space and a 1,200 s watchdog
per executed construction/solve. Compilation is separate. Source/input
hashes, commands, exits and partial outputs survive failure. Existing jobs
are reconciled before any retry. A later retry has a new record.

Construction/root diagnostics and process durations are not an IC speedup.
A successful solver must subsequently collect through rank or the frozen
cap, recover and verify the target and all usable logs, and receive a
separately frozen complete-cost protocol: charged lifts/setup, polynomial
construction, all failed solves, roots, group checks, matrix work, recovery
and verification; A/A plus five paired rounds, strongest matched baseline
and rho, and the 0/0.1/1 GAE band. The fully symmetric pure-L0 S5 method is
a baseline obligation before a best-baseline claim. No leaderboard or
benchmark-index promotion occurs without the repository validation gate.

## Prior work and interpretation

Permutation and low-order-torsion invariant methods predate this round:
[Faugere--Gaudry--Huot--Renault](https://eprint.iacr.org/2012/199) and
[Faugere--Huot--Joux--Renault--Vitse](https://doi.org/10.1007/978-3-642-55220-5_3).
The latter requires the invariant generators used in descent to be defined
over the intended base field. The Semaev resultant construction and
partially symmetrized extension-field setting are also established work
[Joux--Vitse](https://eprint.iacr.org/2010/157).
This round tests the concrete mixed image mechanism and complete costs;
its polynomial degrees alone establish neither novelty nor a speedup.
