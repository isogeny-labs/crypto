# Round10 evidence and replay

[Report](REPORT.md),[PDF](report.pdf),[proof map](PROOF_MAP.md),
[ledger](ledger.json),[protocol](PROTOCOL.md),[validation](VALIDATION.md).

Four explicit quadratic F2 polynomial systems give closed affine models
of specified descended dense opens,with denominator-norm saturation by
inverse witnesses. The complete proper projective quotients are not yet
embedded. The divisor x0*xinf is complete on22 closed cusps;all eight
pole/cuspidal strata are computed. Seven noncuspidal zero divisors remain
open. Four hundred supersingular diagonal labels leave460 total
multiplicity unresolved. Twenty-three new named Lean proofs pass.
No IC efficiency or rank increase is claimed.

From this directory:

```sh
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
./REPLAY.sh
```

All thirteen new JSON artifacts replay byte for byte. Four historical
JSON inputs suffice;the corrected counter-label record is canonical and
its original metadata attempt remains in the archive. Round09 replays
unchanged. Full source/raw hashes are bound in ledger.json. Native
assembly receipts record-* are excluded from that raw binding to avoid
circularity,and included in the archive member manifest. Earlier archives
are unchanged. Deterministic repack:

```sh
tar --sort=name --mtime='2026-10-19 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > /tmp/koblitz-round10-runs.tar.xz
cmp runs.tar.xz /tmp/koblitz-round10-runs.tar.xz
```

Program operations and the shared reduced basis powers specify every F2
scalar polynomial. q8c-expanded-F2-equations.json gives72 literal examples.
No Boolean equations are present. The F2 algebra/scheme interpretation,
Tate cusp geometry and Hurwitz torsion-module bridge remain written in
PROOF_MAP.md;the global geometry is not fully Lean. The failed saturation
inference and incomplete degree-exhaustion test remain recorded as such.

From the repository root:

```sh
cargo test --release --offline --locked --manifest-path research/koblitz_modular_ic_20261018/native/Cargo.toml
tools/lean/proof-gate.sh research/koblitz_modular_ic_20261018/lean
```

For current rendering use the patched isolated command below instead of
executing the builder in this archived directory. The historical build used
`build_report.py`. Python only renders frozen ledger/report content.
Every PDF page is inspected. Same-agent replay/parser checks share declared
primitives;no external audit or scheduled background experiment is claimed.


### Patched, isolated report reproduction (2026-11-15)

The environment and output hashes above describe the frozen historical build.
For current reproduction, install `report-requirements.txt` in a scratch environment
and run the following from the repository root, choosing a new directory outside
the checkout. This copies the pinned builder and inputs; it never renders over
the archived outputs.

```sh
python tools/reproduce_archived_reports_v1.py --round 10 --output /absolute/new/reproduction-directory
```

The original `PyMuPDF==1.26.5` requirements bytes, their SHA-256, and the
source revision are retained in `tools/report_reproduction_v1.json`. The active
requirements use `PyMuPDF==1.26.7`; Typst remains at 0.15.0. Do not invoke
`build_report.py` in the frozen round directory. See `docs/archived-report-reproduction-v1.md`.
