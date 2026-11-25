# Research figures

These figures render the report's stated scope and already accepted native
counts. `render_figures.py` performs document rendering only. It does not run
the selector, elliptic arithmetic, FFT, certificate issuer, or any benchmark.

| File pair | Figure dimensions | PNG dimensions |
| --- | --- | --- |
| `route.svg`, `route.png` | 8.8 × 8.2 inches | 2640 × 2460 pixels at 300 dpi |
| `counts.svg`, `counts.png` | 8.8 × 5.9 inches | 2640 × 1770 pixels at 300 dpi |

SVG is the preferred print source. The PNG versions use the identical layout.
Use at approximately 6.7–7.0 inches wide in the report, with the route diagram
on a page where its labels can retain their full width.

## Data provenance

`selector_counts.csv` is a direct transcription of rows k=5,...,12 from the
accepted file `native/attempt02/evidence/attempt-02/normal.stdout` in the parent
working task. The raw file's SHA-256 is
`8cd8cb2de7d879a15a1146b34e752558d264e5d70f43cf8ec9acb3ff60d3c380`.
Its schema is `pplus-native-selector-check/v2`, and its recorded status is
`all_required_checks_passed`. The CSV preserves the original field names.
Chart N reads `root_count`; chart B reads `curve_columns`. Signed affine and
zero-y counts are retained in the CSV for context, not plotted as extra series.
No half-density estimate or derived speed assertion is displayed.
[pcc:unverified]

## Recommended captions

**Route.** A norm-one-torus coordinate construction supplies the full Dickson
fibre, which must still be filtered into a rational elliptic factor base.
The qualified binary line FFT acts only on the full unfiltered fibre's
quotient algebra. Retained S3 relation extraction, sign reconciliation,
useful-rank accumulation and scalar recovery remain separate obligations;
the arrows do not assert a torus-to-elliptic group homomorphism.
[pcc:unverified]

**Counts.** Exact recorded full-selector counts N and rational x/negation-orbit
column counts B for the Prime31 curve y²=x³−3x+21 at depths 5–12, copied from
accepted selector attempt 02. The horizontal axis is logarithmic to keep every
count pair readable; each marker is labelled with its recorded integer count.
These counts measure neither relation yield nor algorithmic speed.
[pcc:unverified]

## Rendering

Run `python3 render_figures.py` with Matplotlib available. Python is used here
only for static figure rendering from the recorded CSV and stated diagram text.
