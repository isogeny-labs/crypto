# Fixed published larger Frobenius conductor

The original goal remains an explicitly evaluated ordinary-curve isogeny
crossing a certified prime endomorphism-conductor ratio of at least 80 bits,
with repeated matched scalar and ECDLP measurements. No substitute criterion
is introduced here.

This separate development fixture reconstructs the fixed published index
7351 from [OEIS A101087](https://oeis.org/A101087), and attempts a complete
PARI ECPP certificate. The source separately labels its later entries as
probable primes. Neither those entries nor a source's prime label will be
treated as a certificate. No parameter search or benchmark is run here.

## Derivation and acceptance criteria, frozen before checks

For E: y²+xy=x³+1 over F₂, direct counting gives four points and t=-1.
Its Frobenius π obeys π²+π+2=0, of fundamental discriminant -7.
Thus Z[π] is already the maximal order. The ordinary geometric endomorphism
ring equals that order and stays the same under finite base extension.
Over F_(2^n), π^n has trace t_n and

    t_n² - 4·2^n = -7 U_n²,
    U_0=0, U_1=1, U_n=-U_(n-1)-2U_(n-2).

Consequently the Frobenius conductor is |U_n|; the source curve's
endomorphism conductor is 1. This does not identify the endomorphism
conductor with the Frobenius conductor on that same curve. The missing
lower curve and evaluated map remain separate obligations.

Check only the published fixed n=7351. Require agreement of the recurrence
and matrix power, the exact discriminant identity, complete certificate
generation, replay on a fresh GP process, and rejection of a changed
certificate subject. A C++/GMP oracle checks the same integers through
quadratic-ring binary powering. Any timeout, error, stderr or missing
certificate prevents a certified-prime claim. These are the pre-check
acceptance criteria; the completed outcomes are recorded below.

Development limits: at most 300 seconds for the fixed certificate attempt,
512 MiB PARI stack, no paid resource and no automatic retry. Build and
arithmetic proof checking are local development validation of this published
fixture, not a cloud performance experiment. No elapsed time becomes a
scalar-multiplication or ECDLP measurement.

Sources checked: the existing 13 November K-571 record-search study and its
KKM source; [A001607](https://oeis.org/A001607),
[A107920](https://oeis.org/A107920),
[A101087](https://oeis.org/A101087), and its author
[Michon](https://www.numericana.com/answer/numbers.htm#m);
[Sutherland Lecture 13](https://math.mit.edu/classes/18.783/2023/LectureNotes13.pdf)
for ordinary invariance and endomorphism orders; the official
[PARI primecert documentation](https://pari.math.u-bordeaux.fr/dochtml/html-stable/Arithmetic_functions.html#primecert).
The index convention is U_n as in A001607, confirmed by the overlapping
small values; A077020's currently displayed offset differs by two.
KB unavailable: no MCP service is exposed and scripts/research-kb status
returns 127. No novelty or world-record claim is made.

The original 80-bit source/kernel, all existing proof subjects, and all
measurement requirements remain unchanged. S3 publication remains blocked
as recorded in ../wide-harness-20261118/publication/status.json.

## Arithmetic checkpoint

The fixed reconstruction produces a 3673-bit Frobenius conductor. The native
GP matrix/recurrence and C++/GMP quadratic-ring oracle agree on its exact
integer and trace. PCC-20261118-5de5c2 binds these outputs and the frozen
source/binary manifest as an unsigned producer statement, not_covered and
unverified. This checkpoint makes no primality assertion. [pcc:unverified]

The complete integer is retained in attempt-a/arithmetic.json. Neither a
binary-field basis, a lower endpoint, a torsion kernel, a point image nor a
measurement session is supplied by this reconstruction. The absence of those
objects is retained even if the prime certificate attempt succeeds.

### Proof map

| Statement | Evidence or obligation |
|---|---|
| Base model has trace -1 | PARI count over F₂; four points |
| Ordinary and geometric maximal order -7 | Trace odd; fundamental base discriminant; Sutherland 13.3/13.8/13.20 |
| Frobenius conductor after extension | Exact trace/discriminant identity and independent ring arithmetic |
| Conductor primality | Complete ECPP certificate accepted; fresh-process replay accepts and changed subject rejects |
| Lower endpoint conductor and map | Not constructed or evaluated |
| Formal Lean theorem | No lean/lake executable exposed; no formal proof asserted |

No asymptotic or runtime claim follows from these arithmetic checks. The
future conductor-changing map still needs actual construction and evaluation.

## Completed fixed-example qualification

The exact conductor is a **3673-bit prime with 1106 decimal digits**.
PARI generated and accepted its complete 306256-byte ECPP certificate.
A separate GP process recomputed the subject, accepted that certificate and
rejected a changed subject. Both processes exit 0 with empty stderr.
The C++/GMP oracle agrees on the full trace and conductor; its small fixed
controls and the K-571 overlap also pass. [pcc:unverified]

PCC-20261118-a08131 binds the summary, certificate, replay, generation output
and pre-check manifest. This remains an unsigned producer statement,
`not_covered` and unverified by the independent PCC execution mechanism.
The mathematical certificate was replayed on the same development host,
not on an independent host. [pcc:unverified]

- [Exact conductor integer](conductor.txt), also in
  [arithmetic.json](attempt-a/arithmetic.json).
- [Complete ECPP certificate](attempt-a/conductor.cert), SHA-256
  `0d8436fabf198eac552846f934cf94483befc1e84b7341ec365530c510bd720a`.
- [Fresh-process replay](attempt-a/replay.stdout) and
  [qualification summary](summary.json).
- [Illustrated report](reports/report.pdf), editable Typst and recorded-size
  graph; its sizes are not runtime ratios or evaluated-crossing records.

The conductor equals the whole |U_7351|, rather than a known prime factor
of a composite Frobenius conductor. The current source's endomorphism
conductor remains 1. No second curve, kernel, isogeny or endpoint measurements
are supplied here, and no world-record assertion is made. The original
80-bit construction requirement stays open. [pcc:unverified]

## Reproduce

The source is PARI/GP 2.17.2; the C++ oracle uses GMP. Use a fresh directory
because GP `write` appends. From the study directory:

```sh
study=$(pwd)
run_dir=$(mktemp -d)
c++ -O2 -std=c++17 -Wall -Wextra -Werror ring-oracle.cpp -lgmpxx -lgmp -o "$run_dir/ring-oracle"
"$run_dir/ring-oracle" > "$run_dir/ring.jsonl"
cd "$run_dir"
timeout 300 gp -s 512M -fq < "$study/check-fixed.gp" > certificate.stdout 2> certificate.stderr
test ! -s certificate.stderr
test -s conductor.cert
timeout 60 gp -s 512M -fq < "$study/replay.gp" > replay.stdout 2> replay.stderr
test ! -s replay.stderr
```

Also require both GP exits to be 0, the expected JSON completion/replay
fields to be true, and exact equality of the ring oracle's n=7351 trace
and conductor with arithmetic.json. Exit status alone is insufficient.
The current attempt-a retains every raw outcome. A post-check hash pass,
with manifest paths resolved from the repository root, preserves the pinned
sources/executables. An initial check from attempt-a used the wrong working
directory and failed to resolve relative paths; the root check passes without
changing a subject. No retries or performance measurements were performed.

## Publication and exporter checks

The fixed-case files satisfy the existing storage size/suffix policy.
The committed-head branch check still fails on the unchanged 318420-byte
wide-harness runs.tar.xz archive, which requires the inaccessible S3 store;
the PR remains draft. No new bucket request or unchanged workflow retry was
made for this fixture. The original archive and all proof subjects retain
their exact hashes. [pcc:unverified]

The committed-state exporter emits
`conductor.fixed_lucas.n7351.prime_bits=3673` with an explicit
"no evaluated crossing; PCC unverified" label. Its scoped positive and
missing-source controls pass. The full exporter command exits 1 on an
unchanged malformed docs/ic/progress-timeline.json, line 565 column 6.
That legacy JSON is byte-identical to this commit's parent. Retained
metrics-full-partial.jsonl is incomplete output, not a successful full
export; metrics-qualification.json records the scoped result. [pcc:unverified]
