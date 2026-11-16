# Compact graph-kernel equations

Read report.md for hypotheses, proof and exact scope; report.pdf includes the diagram and exact toy-algebra dimension chart. Input.json contains the P-192 curve, subgroup order and point pair. No target logarithm is supplied to build.py.

Independent validation: run `python3 verify.py` and `python3 verify_elimination.py` in a disposable copy of this directory. These commands write verification records. Standard Python 3.9+ is sufficient for both checkers. To regenerate the producer certificate and its symbolic degree check, run `python3 build.py` with SymPy installed, then `python3 eliminate.py`. PDF rendering additionally needs typst and pymupdf. Preserve the sealed source directory; use a copy for regeneration.

Check manifest.sha256 before replay. Dependencies.sha256 binds the historical origin of the point fixture; no earlier-round import is needed by these scripts. If that earlier certificate is held in the repository artifact store, hydrate it with tools/artifacts before checking the provenance dependency.

The compact system and its projected subgroup are specified over the base field. The toy elimination enumerates 32 assignments; no P-192 elimination or quotient evaluator is supplied. Scientific statements are recorded as an unsigned statement claim, pending trusted signed verification. [pcc:unverified]
