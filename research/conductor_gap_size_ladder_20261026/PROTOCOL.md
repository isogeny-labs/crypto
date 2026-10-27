# Broader conductor-gap sizes and confirmation precision

Date: 2026-10-26. Version 1; freeze before the parameter search. This is a
parameter-feasibility and experimental-design task, not a solver campaign.
The user requests substantially broader conductor gaps and a size capable of
supporting a high-confidence conclusion. Neither a larger integer nor more
repeats on one curve satisfies that request by itself.

## Deliverables and boundaries

1. Search a reproducible four-rung candidate ladder at **40, 47, 54 and
   61 field bits**, depth four, D = -11, -19, -43, -67, eight different
   prime-gap classes per discriminant/rung: 32 classes per rung, 128 total.
   Require cofactor at most 64 and actual prime-subgroup bits at least
   field bits minus six. Retain every unsuccessful cell and search count.
2. Separately search **112, 160 and 256 field-bit parameter classes**,
   D = -19 and -43, depth four, trace -2, cofactor four: one per cell.
   These six classes target subgroup sizes 110, 158 and 254 bits. Certify
   primality using explicit Pocklington witnesses, not probable-prime tests
   alone. These are parameters; no exact curve model, point, floor,
   isogeny, or successful logarithm solve is claimed.
3. Independently verify all retained parameter certificates. Publish actual
   gap and subgroup ranges, a confidence/power sensitivity calculation,
   sources, raw compact outputs, checks, a graph and inspected PDF.
4. Freeze the confirmation objective and pilot rule below. Execution of
   that solver campaign remains pending, with its construction, maps,
   execution feasibility and fresh pilot stated explicitly.

Only public synthetic known-answer work is in scope. Preserve all historical
archives and reports. No paid resources, skipped checks, unrelated merges,
new solver measurements or speed/hardness promotion in this parameter task.

## Parameter search fixed before output

The ordinary family satisfies t² - 4p = -d (2^depth ell)². For the four
native-size rungs, use p = tp² + d 4^(depth-1) ell² and t = 2tp.
For each cell scan ell down from floor(sqrt((2^bits-1)/(d 4^3))).
Test only primes; scan tp = m ell - 1 of odd parity in increasing
(absolute value, signed value), within the field-bit bound. Take the first
eligible trace for each accepted ell. Require p prime, ell prime,
order = p + 1 - t = h r with r prime, h <= 64, r² > 16p,
ell^4 > p, gcd(t,p)=1, and actual r bits >= bits-6.
Accept eight distinct (p,t) classes per cell, never duplicate one class as
an additional independent sample. Maximum 500,000 ell candidates per cell.

For upper parameters, let ell = k 2^m + 1 with
m = ceil(bit_length(ell_max)/2), scanning k down from its admissible maximum.
Set p = d 64 ell² + 1, t = -2, r = d 16 ell² + 1, h = 4.
Require exact requested field bits and subgroup bits = field bits-2.
Probable-prime checks only filter candidates. Prove ell prime with known
factor F = 2^m of ell-1 and p,r prime with F = ell² of p-1,r-1.
For each N, find the first a in 2..=128 such that
a^(N-1) = 1 mod N and gcd(a^((N-1)/q)-1,N) = 1, with F² > N.
Pocklington then applies recursively from q=2. A missing witness rejects
the candidate; total examined-candidate counts are retained. Maximum 3,000,000
k candidates and 120 seconds per upper cell; 900 seconds total search.
An exhausted cell remains incomplete, not evidence of nonexistence.

The deterministic seven-base Miller–Rabin theorem for integers below 2^64
certifies the native-size p,r,ell. Parameter checks alone do not certify
End(E), the actual group order of a supplied model, or transport. Do not
assign an ICV1 identity to a curve that has not been constructed.

## Meaning of high confidence

The proposed primary estimand is, separately at each of the four native
size rungs, the geometric mean of class ratios: average the three S2
crater means in a class, divide by its S1 crater mean, then give each
class equal weight. All costs are cold counted S = GAE/sqrt(r), with
the same 0, 0.1 and 1 GAE band over emitted unpriced counters on every side.
This is a fixed designed cohort, not a random sample of ordinary curves.
Family, individual-class, depth, wall-time and algebraic-method claims
are outside this four-comparison primary objective.

Use **99.75% intervals per rung**, so the union bound gives **at least 99%
simultaneous coverage over four intervals**, provided each interval has
its stated coverage. Bootstrap coverage is approximate, so the actual
method's calibration must be checked, not asserted from the nominal label.
Equivalence requires all four intervals to lie wholly inside [0.95,1.05]
and every correctness, completeness and control gate to pass. A failure
to detect a difference does not establish equivalence. Wider intervals or
censoring remain inconclusive; a difference is reported as a difference.

Pilot: after model/order/map certification and a recorded feasibility
check, use 32 fresh distinct targets per crater and two independent rho
seeds, on all selected classes, excluded from confirmation. This is
32 x 4 x 2 x 128 = 32,768 one-target pilot solves, plus full replays.
If this pilot cannot execute within the existing local envelope, preserve
that resource blocker; do not quietly reduce the requested rung or call a
small subset the complete pilot. No such pilot is launched by this task.

Provisional confirmation allocation: **32 classes/rung x four craters x
1,000 fresh targets/crater x two seeds/target x four rungs = 1,024,000
independent one-target solves**, plus separate correctness replays and
controls. This is a planning scenario, not a promised minimum or a launched
job. Pilot estimates and their uncertainty must determine the fixed count
before confirmation. Target at least 90% probability of passing all four
equivalence gates at true ratios in [0.99,1.01]; also show sensitivity at
the historical 1.01112520 ratio and at 1.03, and variance inflation by 2/4.
If the pilot does not support the precision/coverage/power objective or the
resource cost, leave confirmation pending and revise a new protocol before
running it. Never sample until an interval happens to pass.

Use disjoint pilot/confirmation target namespaces, verify distinct nonzero
scalars, and carry the same target scalars through certified maps. Each
target/seed invocation owns its tables. Rho seeds differ across repetitions;
equal numeric seeds across coordinate-dependent walks are not a fixed-walk
transport control. Randomize complete class/curve execution blocks, not
just method order inside one workload. Retain class, curve, target and
seed dependencies in inference. Show class-level heterogeneity and A/A,
coordinate and plain-rho positive controls. All records, failures,
censoring, independent scalar checks and exact replays must be retained.

The planning calculation reads the old native design.json variance model.
Use normal/delta-method scenarios and a union-bound lower bound on joint
power, without assuming rung independence. No transfer of those variance
estimates to larger subgroups is established. Use them for sensitivity
planning only. No exponent or cryptographic-size hardness inference.

## Construction and execution obligations still open

For every retained parameter class, construct the exact S1 model and three
distinct S2 endpoints with a common source generator, certify ordinary
orders, conductor strata and depth, and verify explicit maps, their duals,
G and Q transport, nontriviality on the r subgroup, and the scalar law.
Charge construction and both target evaluations separately; do not treat
matching orders or j invariants as a map certificate. Certify actual model
and representation identities before ecbench execution. In particular,
upper parameter certificates do not construct the previously missing
non-rational-kernel floor or provide a fast point-evaluation backend.

The old 96-bit result already certifies a floor at gap
125000000302579; old 53-bit ECDLP comparisons cover gaps
33555419..35282963 but have distinct transfer/timing limits. Broader native
rungs must not be advertised as the first comparison at gaps of millions.
The upper parameter track aims beyond that largest certified floor gap,
while retaining floor/map construction as pending.

## Exact mechanical dispatch

From the repository root, build before running (no concurrent timed jobs):

```sh
g++ -std=c++17 -O2 -Wall -Wextra -Werror research/conductor_gap_size_ladder_20261026/feasibility.cpp -lgmpxx -lgmp -o target/conductor-gap-size-plan
g++ -std=c++17 -O2 -Wall -Wextra -Werror research/conductor_gap_size_ladder_20261026/verify.cpp -lgmpxx -lgmp -o target/conductor-gap-size-verify
```

After committing this protocol and sources, the director compiles one card:

1. `target/conductor-gap-size-plan --self-test` (30 seconds).
2. `target/conductor-gap-size-plan research/conductor_gap_crater_design_20261025/design.json research/conductor_gap_size_ladder_20261026/results` (930 seconds).
3. `target/conductor-gap-size-verify research/conductor_gap_size_ladder_20261026/results/candidates.csv research/conductor_gap_size_ladder_20261026/results/verification.json` (120 seconds).

Success codes are [0] for all steps, no retries; results directory must not
exist. Bind this protocol, both source files, old design.json and both
binaries by SHA256. Card logs/receipt/assessment stay under cards/. The
executor runs only the verbatim card prompt; main relays the output to the
director for assessment. This search records arithmetic certificates,
not runtime comparisons; no ECDLP solver runs in any step.

Local validation before push includes native controls, independent parameter
verification, `cargo test --release --lib`, relevant site/validation-gate
tests, artifact/source hashes, the large-file check and PDF inspection.
Conductor was attempted: CLI absent; no shared lease is claimed. Existing
GitHub access and local resources suffice; no credentials are provisioned.
