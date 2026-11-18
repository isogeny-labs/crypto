# Complete certificate for the larger fixed conductor

The exact Frobenius conductor at the fixed published index 9551 is a
**4774-bit prime with 1438 decimal digits**. The full 506338-byte PARI ECPP
certificate is accepted; a fresh GP process recomputes its subject, accepts
replay and rejects a changed subject. Native C++/GMP ring arithmetic agrees
with GP recurrence and matrix arithmetic on the complete trace/conductor.
PCC-20261118-0282a8 binds the evidence; producer standing is unsigned,
unverified/not_covered. [pcc:unverified]

The curve is E: y²+xy=x³+1 over GF(2^9551). Its geometric endomorphism
conductor remains 1. Its Frobenius conductor is |U_9551|, where
U_0=0, U_1=1 and U_n=-U_(n-1)-2U_(n-2). The exact integer is in
[conductor.txt](conductor.txt); this fixes the subject without rounded values.
This is the largest certificate-backed Frobenius conductor in the two fixed
Lucas cases checked here, not a world-record assertion. [pcc:unverified]

- [Complete certificate](attempt-b/conductor.cert), SHA-256
  `8e644db3eb73426d96209c5e4547d07958b2a56a4f4e79049a7103dc73521be9`.
- [Fresh replay](attempt-b/replay.stdout), [qualification](attempt-b/qualification.stdout)
  and [summary](summary.json).
- [Illustrated report PDF](reports/report.pdf), editable Typst and graph.
- [Immutable original scope](README.md), [staged continuation scope](STAGED-SCOPE.md)
  and [primary-source review](sources.json).

## Actual attempts

| Attempt | Cap | Actual outcome |
|---|---:|---|
| A: direct full ECPP | 300 seconds | GP exits 124; no certificate; qualifier exits 1; no replay |
| B: staged ECPP | 600 seconds cumulative | Four partial chains saved; full certificate generated/accepted; qualifier exits 0 |
| B: fresh-process replay | 60 seconds | Exit 0, empty stderr; subject matches, accepts full certificate, rejects changed subject |

Generation/replay stderr is empty for the successful attempt; source and
executable hashes match before/after both attempts. The original timeout is
retained unchanged. Partial certificates alone were never accepted as full
proofs. This is bounded local development qualification, not a timing session;
no certificate duration is used as scalar-multiplication/ECDLP performance.
[pcc:unverified]

## Original requirement status

| Requested deliverable | Status |
|---|---|
| Large prime conductor arithmetic | New fixed case certified and replayed |
| Minimum 80-bit prime endomorphism-conductor gap between explicit endpoints | Original source/kernel retained; explicit lower endpoint/conductor evidence absent |
| Quotient, map and non-kernel images | Not supplied by this arithmetic fixture |
| Dozens of points, repeated scalars, several fields | Separate existing small-map controls and qualified paired plans; no new endpoint measurements |
| Scalar/ECDLP/IC comparison | No new session; wide-field IC, wall reference and admission still open |
| GitHub/S3 | Existing draft PR extended; earlier wide-harness archive still blocked on S3 |

The next original-goal obligation is the complete vertical construction and
actual transport images. All map setup/evaluation and endpoint timings stay
unknown. No target or benchmark job is active after the fixed proof checks.
A same-host mathematical replay is not an independent signed execution receipt.
[pcc:unverified]

## Reproduction and historical correction

Use PARI/GP 2.17.2 and C++17/GMP. Compile ring-oracle.cpp and pin sources plus
the native executables. The two frozen shell qualifiers take a repository
root and fresh output directory; direct mode also takes the oracle binary.
Run direct mode only to reproduce the retained original capped attempt; staged
mode uses its successful native arithmetic output and does not rerun it.
`primecert(C)` extends the saved partial chain; only `primecertisvalid` on the
full final certificate satisfies the gate. GP writes append, hence fresh paths.

The pre-call manifests retain the immutable README and source identities.
The new result report does not alter them. collect-metadata.py only joins raw
receipts; its argument is the temporary receipt directory. Plots use recorded
metadata only. The original and staged scopes explicitly record the cap change.

The draft arithmetic statement PCC-20261118-9456e2 mistyped the decimal count
as 1437 while its bound JSON correctly recorded 1438. It is preserved and
explicitly corrected by PCC-20261118-31707e. The final claim reads its counts
from the checked summary. This was a producer prose error; the native integers
and oracle agreement were unchanged. [pcc:unverified]

Canonical prior graphs and the original conductor metric stay unchanged;
a separate n9551 metric reports this fixed prime with the explicit label
"no evaluated crossing; PCC unverified". No benchmark or leaderboard row moves.


Scoped metric admission controls pass for both retained fixed cases; missing
new source and five negative controls emit no new value. PCC-20261118-c0f327
binds those development controls and exporter source, unsigned/not_covered.
The full exporter retains its separately recorded malformed timeline JSON
failure; it is not called a passing command here. [pcc:unverified]


All attempt records stay loose because the frozen qualifiers, replay,
summary hashes and PCC subjects resolve these exact paths. This uses the
named-record exception in AGENTS.md §13a. Earlier committed evidence is
neither repacked nor renamed; the new fixed-case files satisfy configured
storage thresholds. The previous wide-harness S3 violation remains separate.
[pcc:unverified]
