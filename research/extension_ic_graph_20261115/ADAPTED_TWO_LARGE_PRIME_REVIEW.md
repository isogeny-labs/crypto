# Adapted two-large-prime cover sieve component

Selected after published27counter validation. This is the classical adapted
Joux–Vitse construction, not a novelty claim. [pcc:unverified]

Sources checked(KB): MCP absent; wrapper status and exact/negative/application
queries unavailable(no executable), /tmp/ic-adapted-2lp-kb-*.log. Primary:
Joux–Vitse author PDF, https://www-fourier.ujf-grenoble.fr/~viva/research/articles/CoverDecomp.pdf,
SHA2567ca88c54ebfcb6fac8d05d313bfe8c26d3cb133081c55c4ab6ea4c1b45729b38,
§3.3/§6. Read existing jv_sieve polynomial/sieve/materializer, factor_base,
large_prime_filter certificate/replay and23/25fill negatives. [pcc:unverified]

Fix a prefix of the canonical ascending Fp-abscissa base as the small base.
The existing quadratic-parameter sieve accepts m-2small roots rather than
m. Materialize the degree-m polynomial, peel every distinct small root,
then solve its remaining degree<=2factor overFp. Refuse nonsplit/repeated
factors, wrong degree, missing points, exceptional ordinates and incompatible
covers. The omitted roots are actual large-prime vertices identified by
Fp abscissa. They are not irreducible Fp² quadratic factors. [pcc:unverified]

Verify every original signed divisor relation in the Jacobian before graph
use. Bind its small/large split and logarithm modulus, recheck those inputs,
then use the existing odd-prime graph filter. Replay every complete-row
certificate from the original inputs before emitting small-base SparseRel
rows. These rows are in logarithm coordinates modulo the caller's prime
order; the raw Jacobian divisors may need cofactor projection. The constructor
checks each factor-base point and its Mumford representation against the
cover. [pcc:unverified]

Public library API is jv_sieve::two_large_prime::{TwoLargePrimeSieve,
TwoLargePrimeRelation,FilteredTwoLargePrimeRelations}. Existing default
entrypoints, threshold, root-factorization RNG consumption and serialized
outputs remain. Limits: m8..16,p<=1048576,4096input relations and4096row/source
terms. The component probe further bounds prime p<=1511 and uses fixed degree9,
incidence enumeration/shared arrays with separate enumeration/extraction RNGs.
It hashes the entire B/line cohort so paired arms must see identical lines.
[pcc:unverified]

The component reference is the existing full-smooth finder on the SAME
reduced small base and B cohort. Comparing yield against the old entire base
would be misleading: all accepted two-LP polynomials still split overFp
and are a subset of its full-smooth polynomials. The expected benefit is
cheaper small-base relation search plus graph elimination of omitted points,
with graph cost/fill included. Full-factor-base construction, extra source
verification and certificate replay are charged in the component boundary.
[pcc:unverified]

The probe emits original source relations and complete graph certificates,
phase primitive counts, selected sampler counts, counts of refused/duplicate
candidates and exact cohort identity. It leaves complete costs/ranking false
and individual-log descent/recovery unexecuted. Every uncounted native path
remains explicit. No original-DLP or rho-parity result follows from component
rows or their count; rank and full-pipeline descent/recovery remain obligations.
No measurement protocol is selected by this source review. [pcc:unverified]

Planned meaningful source checks: every quadratic overFp19 with nonzero
leading term against exhaustive evaluation; actual weak-cover partial rows
against existing full polynomial factorization and Jacobian verification;
binding/tamper/modulus/cover refusals; inherited full-smooth count, group,
graph-rowspace and worker API regressions. Native checks subsequently pass:
21sieve/graph tests(twoexisting ignored), including exhaustive quadratic
and actualcurve/fullfactorization controls;12generic/MSTfilter tests;
oneexecuted publicworker test; actualproduction probe/worker builds and
strictlibrary/twoexamplesClippy. The standalone component qualifier passes
12tests/build/strictClippy, including independent modular rank/replay,
pseudoprime, tamper and pairedcohort controls. Three root source hashes
stay unchanged through final compilation. FullDLP and independent signed
admission remain open; Amendment23 separately freezes the actual diagnostic.
[pcc:unverified]
