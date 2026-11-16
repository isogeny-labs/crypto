# Compact full mixed-kernel presentation

Start with report.md or report.pdf. The equations are generated from the bound P-192 point input in ../round52-p192/input.json; no target logarithm is read. The earlier round52 model.py supplies the complete addition predicate. Hydrate earlier artifact-store inputs if the checkout uses manifests.

Replay in a disposable copy of the study layout: `python3 compile.py`, `python3 build.py`, `python3 verify.py`. Standard Python 3.9+ suffices. compile.py writes the blueprint; build.py constructs actual F_(11^3) controls and checks compiled witnesses; verify.py uses separate finite-field, curve and pairing arithmetic. These commands write records, so preserve the sealed source directory. PDF rendering needs typst and pymupdf.

To export the actual P-192 polynomial stream, run `python3 compile.py --emit full_mixed --output /tmp/mixed.jsonl`; compare its SHA256, line count and byte count with export-verification.json. Each line is an expression tree for a polynomial set equal to zero, with +,-,* operators, integer coefficients interpreted in F_p, and v-numbered variables. P192-blueprint.json names the output-point variables and binds the input hash.

The preserved attempt1 files describe the initial full-p/binary-power presentation. Amendment1.json records the two exact substitutions. The attempt2 logs retain an intermediate Python indentation error. Neither historical attempt should be silently overwritten.

Scientific standing is an unsigned statement record pending a trusted signed verdict. The geometric argument is written and the field/kernel controls are finite; P-192 elimination and quotient evaluation are open. [pcc:unverified]
