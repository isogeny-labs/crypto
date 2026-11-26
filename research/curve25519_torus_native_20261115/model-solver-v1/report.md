# Complete-model replay for Curve25519 spectra

The native adapter is implemented: all-input model extraction, exact binding /
field validation, complete gate reconstruction and original indexed relation
projection. Twelve supplied synthetic model responses pass, as do small native
SAT/UNSAT protocol and output-cap controls. Synthetic model replay does not
establish coefficient discovery. [pcc:PCC-20261123-ab3e5b]

One fixed public section query used p101/q3/k8, R=(89,27), Q=[8]R, and 3511
variables/3519 equations/44 unknown semantic inputs. No unknown values were
supplied. The backend reported a timeout at its frozen 5000ms limit before any
model. The adapter retained stderr/pipe refusal, sent no get-value request and
did not retry. This supplies no absence, hardness or exponent inference.
[pcc:PCC-20261123-ab3e5b]

The unchanged 76-input compiler archive and new 9-input delta roundtrip to all 85
source inputs; seven new hashes are compiled in. The native executable's
pre/post hashes match and all captured streams roundtrip through the 37368-byte
archive. Focused release tests, rustfmt and strict Clippy pass locally. All
standing is unsigned/not_covered. [pcc:PCC-20261123-ab3e5b]

Next, qualify a direct-point model adapter on the same target, then derive a
concrete block-specific elimination operation with exact projection and fully
charged costs. Standard BitSplit does not automatically eliminate between
the quotient/Chebyshev and characteristic-polynomial blocks. Comparative
campaigns require a new full-cost EKS protocol. No paid worker or campaign is
running; all original route criteria remain unchanged. [pcc:unverified]

See README.md, PROTOCOL.md, FROZEN.md, REPRODUCE.md and NEXT.md. The inspected
two-page PDF and editable pipeline describe the fixed controls and missing
algorithmic operation; they contain no fitted performance result.
