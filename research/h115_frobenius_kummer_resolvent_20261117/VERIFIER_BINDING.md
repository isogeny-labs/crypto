# H115 independent verifier binding

Binding date: 2026-11-18

- producer repository: `isogeny-labs/crypto`
- producer draft PR: #4625
- frozen producer source revision: `6d8def5473`
- producer source SHA-256:
  `f0644129c8d7c0161cbddac3283f94453983d25b6f6cdeaf327b170c1fc579bd`
- verifier repository: `isogeny-labs/cryptanalysis`
- verifier draft PR: #1739
- verifier source revision: `ed206d8d`
- verifier source SHA-256:
  `e625542931d0b8b6b6b753ce19b495a03b9640c7f1cdb6ba06b3d7d9196e1a37`

The verifier is a standalone Python standard-library implementation. Its
synthetic selftest rejects all eight frozen tamper classes, and its independent
unlabeled structural path reproduces all five extension/torsion fixtures. It
has not read a candidate packet because no H115 scoring run has occurred.

Evidence is admissible only when produced from the bound source after the
checkpoint/image gate and accepted by the bound verifier revision in normal
mode plus copied-packet tamper mode. A later source change requires a new
binding and a new checkpoint before measurement.
