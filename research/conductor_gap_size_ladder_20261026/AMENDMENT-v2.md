# Amendment v2: retain the failed family; complete with D = -67

Date: 2026-10-26. Frozen before the completion search. The v1
[protocol](PROTOCOL.md), results/, card, receipt and stopped assessment are
retained unchanged. V1 source and binary digests are pinned at commit
ee0f5489b8a946ccb6ea870c56b7fea6e8300993.

V1 retained all 128 native-size parameter classes and one D=-43 upper
class at each of 112/160/256 bits. All three D=-19 upper cells exhausted
3,000,000 candidates; the verification step did not run after that stop.
These are parameter-search failures, not failed logarithm solves.

The reasoning tier identified a restricted-family obstruction: for d=19,
t=-2 and cofactor four, p=1216 ell²+1 and r=304 ell²+1. For prime
ell>5, ell² modulo 5 is 1 or 4. In the first case r is divisible by 5;
in the second p is divisible by 5. Both exceed 5 at the requested sizes.
Thus this particular family cannot supply both primes. This does not
exclude D=-19 with another trace, conductor factorization or cofactor.

The native four-case table is in mod5-cases.json. The supporting product
identity has evaluated certificate **IDC1h4e934fe294c7d1ba** in
mod5-identity.json, issued by examples/conductor_gap_size_identity.rs with
32 points and checked on 64 further points. This additional identity check
is over F_(2^61-1); the integer modular argument is the four explicit cases
above. No checked Lean proof is claimed; Lean/lake are unavailable here.

Replace only these three upper cells with **D=-67**, at the same field
sizes, trace, cofactor, scan order, candidate/time guards and certificate
requirements. The native-size 128-class design, all selected targets and
confidence/power objectives remain unchanged. No solver target has run.
Copy the 131 retained candidates from v1; do not repeat their searches.
Verify all 134 parameter classes independently after the three new searches.

The amended native source supports `--complete-v2`, reads the pinned v1
CSV, retains every row, searches only the new D=-67 cells, and writes a
new results-v2/ tree. Reused cells have search count zero in v2, with
stop_reason=reused-v1-certificate; original search counts remain in v1.
No v1 output is overwritten.

Build (before any card execution):

```sh
g++ -std=c++17 -O2 -Wall -Wextra -Werror research/conductor_gap_size_ladder_20261026/feasibility.cpp -lgmpxx -lgmp -o target/conductor-gap-size-plan-v2
g++ -std=c++17 -O2 -Wall -Wextra -Werror research/conductor_gap_size_ladder_20261026/verify.cpp -lgmpxx -lgmp -o target/conductor-gap-size-verify-v2
```

Card CARD-conductor-gap-size-20261026-2 uses exactly:

1. `target/conductor-gap-size-plan-v2 --self-test` (30 seconds).
2. `target/conductor-gap-size-plan-v2 --complete-v2 research/conductor_gap_crater_design_20261025/design.json research/conductor_gap_size_ladder_20261026/results/candidates.csv research/conductor_gap_size_ladder_20261026/results-v2` (930 seconds).
3. `target/conductor-gap-size-verify-v2 research/conductor_gap_size_ladder_20261026/results-v2/candidates.csv research/conductor_gap_size_ladder_20261026/results-v2/verification.json` (120 seconds).

Success codes [0], no retries; bind protocol, this amendment, current source
files, old model JSON, v1 CSV and both v2 binaries by SHA256. Results-v2/,
all card stream paths and the receipt must not exist. Write scope is this
study only, background=true. Stop on any nonzero/timeout/pin refusal.
Commit sources and amendment before execution. Main relays through the
same director/executor/assessment workflow. Analysis, confirmation launch
and any larger execution decision stay with the reasoning tier.
