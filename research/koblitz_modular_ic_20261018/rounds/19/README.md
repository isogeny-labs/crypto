# Round19: controlled poles, mixed sections and coupled elimination

[Report](REPORT.md) · [PDF](report.pdf) · [map search](CM.md) ·
[mixed sections](THETA.md) · [saturation](SATURATION.md) ·
[proof map](PROOF_MAP.md) · [protocol](PROTOCOL.md) · [ledger](ledger.json).

Across both elliptic signs, 335,544,320 controlled-pole x numerators leave
only 64 constant maps under 18 actual incidence constraints. A primitive modulo5 orbit-transfer obstruction is bypassed on the
scalar105 cover: an explicit weighted orbit divisor has polarization
3*105² H and41 evaluated F2 sections on graphs0/2. They have forced
fixed components and48 scalar-kernel noninvariance witnesses; canonical
Pic2 matching and full kernel actions remain open.

Coupled elliptic resultants exclude all four Q28 points. Another 266
pure-source resultants exclude the four Q30 points. Four complete checked
Q32 models on [123,124,126,127] remain unclassified. The integral factor
inclusion, canonical243 sections and whole saturation remain open.

There are 79 new named Lean results with explicit limits in PROOF_MAP.md.
Raw attempts, failures and exact outputs are frozen in runs.tar.xz.
Fresh extraction, native member hashes, source pins and byte replay are
recorded in validation-runs.tar.xz and ARCHIVE_VALIDATION.txt. No IC cost
or efficiency assessment is supplied; historical ranks remain unchanged.

```sh
cd research/koblitz_modular_ic_20261018/rounds/19
sha256sum -c runs.tar.xz.sha256 validation-runs.tar.xz.sha256
tar -xJf runs.tar.xz
tar -xJf validation-runs.tar.xz
sha256sum -c runs.sha256 validation-runs.sha256
./REPLAY.sh
../../native/target/release/koblitz-modular-carriers --round16-verify-sources .
```

The generic source-verifier CLI keeps its historical name. Round18 pins
are checked at a2b8f27, preserving its frozen source snapshot. The renderer
uses recorded native values only and produces the editable diagram,
SVG,150dpi PNG and an inspected PDF. Mathematical and archival evidence
is separated from the missing geometric/implementation obligations.
