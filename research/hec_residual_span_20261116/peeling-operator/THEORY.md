# Sparse dependency operator and target-only certificate

Exact standard Schur algebra, not a new attack or speedup claim. This keeps
symbolic seed scaling from silently turning every point label into a dense
vector. It does not itself solve rank, production or the full10xobjective.

## Dependency graph instead of expanded labels

X consists of seed logs and target k; Y consists of newly derived point logs.
Creation rows are ordered chronologically. After normalizing each creation
pivot they form a unit lower-triangular block L. With closure rows the system is

    L Y + B X = b,
    C Y + D X = d.

Eliminate Y: S X=t, S=D-C L^-1 B, t=d-C L^-1 b. Do not materialize S or
every Ylabel. Apply Sx by sparse Bx, one forward triangular solve, then C/D
products. Apply transpose by reverse dependency accumulation. Both cost
O(E+s+u+r)field operations/storage in the stored dependency/closure terms,
where s=#X,u=#Y,r=#closures,E=total sparse terms. Forward/reverse passes do
not divide; creation normalization was done once. This removes an otherwise
hidden seed-width factor in each application, not a measured total gain.

## Target coordinate without full-rank solving

For a consistent system, k is uniquely determined exactly when its coordinate
vector e_k is in the row space of S. A witness w satisfies S^T w=e_k;
then k=w^T t. Verification uses one transpose application plus a dot product,
even if other seed coordinates remain free. The test driver obtains w via
explicit reference elimination, which is NOT an efficient production solver.
A future black-box method must produce the witness and charge its cost.
Rectangular/rank-deficient systems cannot be replaced by unchecked S^T S:
finite-field isotropic vectors can introduce unwanted kernel behavior.

Unit triangular algebra works over a commutative coefficient ring too, but
field Krylov algorithms do not automatically handle ring zero divisors.
This gate is over the qualified scalar field with known true action; a
future unknown-action operator needs component/CRT validation separately.

## Bootstrap-to-growth budget, explicitly heuristic

Previous exact first-admission bound requires initial pool about q^(1-1/g)
at fixed anchors for nonvanishing first-hit opportunity. It says nothing
about doubling the pool or collecting seed constraints. Under an additional
uniform-incidence mean-field model, new rows per anchor scale as
M^g/q^(g-1), so doubling M0takes Theta(q^(g-1)/M0^(g-1))anchors and
Theta(q^(g-1)/M0^(g-2))point visits. At M0~q^(1-1/g), this is
q^(2-2/g), matching the classical double-large-prime exponent, not beating it.
Adaptive pools depend on target/history; the uniform model is not proved for
them and is not a completion/rank theorem. Concentration might change it.

Sparse black-box LA on s~q^(1-1/g)seeds could also cost about s² up to log
factors under appropriate rank/solver assumptions. Dense Gaussian on an
expanded Schur matrix can instead cost s³. Neither complexity is measured
here. Avoiding dense label expansion is necessary housekeeping before
claiming the smaller seed matrix is cheap. Numerical constants, discovery,
rank losses, certification and final group checks still decide any10xcase.

## Prior art boundary

Improved sieving on algebraic curves,sections1-2, describes both progressively
constructed small-prime sets and the q^(1-1/g)/q^(2-2/g)classical balance:
https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf
Wiedemann1986 motivates black-box finite-field solving in terms of application
cost, not explicit dense matrices:
https://www.enseignement.polytechnique.fr/informatique/profs/Francois.Morain/Master1/Crypto/projects/Wiedemann86.pdf
No priority claim is made for DAG/Schur/inactivation or target row-space
witnesses. The current gate qualifies their implementation on retained rows.
