# Exact embedding degree as a skipped-curve criterion

26 October 2026. Continuing the user's historical-selection objective after
completion of all84 integer factorizations. The historical labels are already
visible. This is an exact finite computation and an exploratory mechanism
check, without blinding, a new p-value, a DLP experiment or a cost claim.

## Literature and question

The [curve-origin authors](https://www.anthropic.com/research/ecdsa-curve-origins)
still identify the skip criterion as unknown. Their observed eleven skips
and ten published controls are the frozen parent historical.tsv population.
[RFC7748 Appendix A](https://www.rfc-editor.org/rfc/rfc7748.html#appendix-A)
defines a modern MOV condition k>(n-1)/100. It dates to2016, so passing or
failing it is not evidence that a1997 generator used it. The exact degree is
k=ord_n(p). The installed PARI2.17.3 manual, pari.desc Function:znorder,
specifies a proved factored group-order multiple as the preferred input.
The official online arithmetic-manual retrieval timed out; use the installed
version-bound source documentation, preserving that limitation.

Previous native checks established only k>=1024. All21 n-1 factorizations
are now proved complete. That permits exact k without further factoring.
Test these two fixed rules, with pass predicting published:

1. Maximal degree: k=n-1.
2. The modern RFC threshold: 100*k>n-1, equivalently index (n-1)/k<100.

Do not tune a cutoff after observing results, add these labels to an untested
classifier, or count a fit as historical attribution. Report every row,
every mismatch, and raw exact degrees/indices. Neither criterion alone is
an ordinary-input curve attack. An exact failure disproves that particular
universal rule; it does not exclude every use of embedding information.

## Fixed computation and controls

Use all21 rows from ../inputs/historical.tsv and their complete minus records
from ../factor_followup_20261026/runs/full84-replay-016/input-factors/.
Recompute n-1, prove n and all factors, and check the exact product again.
Native C++ starts with k=n-1 and removes a prime divisor q while p^(k/q)=1
mod n. Independently cross-check against PARI znorder with the supplied
factored n-1. A separate native executable verifies the recorded k by direct
power witnesses: k divides n-1, p^k=1, and p^(k/q)!=1 for every prime q|k.
That proves minimality. These are separate same-session code paths using
the same native arithmetic library, without independent-host or Lean claims.

Fixed positive controls: ord_7(3)=6, ord_7(2)=3, ord_17(4)=4.
The verifier must reject nonminimal8 and incorrect2 for base4 modulo17,
and a wrong degree for base1. The producer/checker run in fresh processes.
The complete cohort must have21 rows, ten published and eleven skipped;
fail closed on identity, prime, product, degree or schema disagreement.

Compile C++17 -O2 -Wall -Wextra -Werror against the existing PARI2.17.3
prefix and OpenSSL, with the parent's common.hpp frozen beside the sources.
Keep source, binary and exact input SHA-256 values, commands, stdout/stderr,
every exit code, raw certificates and independent verification output.
Use a new runs/exact-001 directory and a successful S3 checkpoint gate
before compilation/execution. Run each native phase with a300-second
watchdog. No paid host or resource increase is needed. Existing local
four-core/16GiB limits apply; no isolation tier or timing result is claimed.

## Admission and delivery

Only publish canonical results after both controls, all21 computed degrees,
PARI cross-checks and separate minimality verification pass. Retain any
failure and its exact cause. Archive raw records losslessly, upload through
the artifact tool and verify a full remote GET and all listed member hashes.
Add a dated report, recorded-value graph and inspected PDF; preserve prior
studies and their PDFs. Commit and open a new regular PR, since2635 merged.
The broader historical explanation remains open unless independent evidence
supports it. The related period-ECPP source lead remains unexecuted on the
named orders and supplies no selection result to substitute for this test.
