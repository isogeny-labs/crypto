#set page(paper:"a4", margin:15mm)
#set text(font:"DejaVu Sans", size:9pt)
#let s=json("run/preflight-summary.json")
= Vieta speed calibration: preserved pilot
17 November 2026 · VIETA-SPEED-PREFLIGHT-20261117

*Registration failure:* canonical export validation reports60 seed mismatches:
the frozen explicit_list cycles seeds by cell, while the instrument follows the
prepublished per-configuration seed rule. The actual rule/source are byte/scope
audited, but strict protocol qualification fails. See REGISTRATION_ERRATA.md.

All72 requested calls succeeded and their complete relation sets replayed on12
distinct inputs. Six fixed configurations each used schoolbook, Euclid/Newton,
half-GCD, half A/A, lookup and half-double on two targets, with no warmups.
Each target population has only one input/configuration; this is a calibration
and control check, not the primary experiment. No calibration input is pooled
with primary data.

The persistent supervisor exited0 at04:32:33UTC after starting04:23:52UTC.
Official isolation records519.346416s local wall,517.957726s user CPU,
0.496793s system CPU,12,800KiB maximum RSS and0.410000s foreign CPU, with
contended=false. PSI is unavailable and unmovable host threads remain; clocks
are belowL2 diagnostic, not admitted speed measurements. Every unified export
row keeps wall_valid=false. These are measured wrapper observations, not
estimated aggregate runtimes.

Every arm constructs the entire certified descriptor and curve-point base;
lookup also constructs its index. Query and construction phases remain
separate. Scalar oracle generation and serialization are excluded and retained.
The same immutable instrument and budgets were used for every arm; each child
exec is fresh. A/A query counts are exactly1× and double query counts exactly2×
in every calibration population/configuration. Median intervals for n=1 repeat
the observation and carry no population-uncertainty information.

#image("calibration-counts.svg",width:100%)

#pagebreak()
#image("calibration-timing.svg",width:100%)

Ratios are the frozen Rust analyzer's precomputed matched ratios. Above1 favors
half-GCD. Count reductions do not establish elapsed speedups. The wide curve
is NIST P224; word62 uses p=4179340454199820289,A=2,B=3 without a prime-order
claim. Rational fixtures are Mobius-conjugate squaring towers, not an
auxiliary-isogeny search.

The frozen native runtime forecast for the subsequent504-call primary panel
is3553.997345s from fixed fixture time plus14 times the larger observed per-input
process+preparation time per configuration. This is a planning estimate, not a
primary outcome or guaranteed deadline. It was used only to fill prospective
runtime metadata before the primary freeze; matrix, sample sizes and stage,
worker, configuration and outer caps were not tuned.

Measured source27bc4bf511cda40f904324901895a246f40a9c75;
qualifier ecf84d40b2638f80c590142d6388c4f3ca95670de60bb5f1625a8c91da142be5;
calibration canonical protocol af0985e46cd0f5b8c95c851e20da0d35a2423400655dd288332d96d6b4689a46.
See run/preflight-publication.json, pre-call-preflight.json, qualifier manifest,
preflight-summary.json, preflight-replay.json, forecast.json and raw preflight/.
No retries or failed rows; exact raw successes remain immutable. Git-only/local
authorization applies, no local S3 operation or remote availability assertion.
Observations remain producer-owned #raw("[pcc:unverified]").
