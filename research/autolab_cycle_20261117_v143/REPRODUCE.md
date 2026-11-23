# Portable C143 reproduction

Use a fresh checkout of this PR. Run `python research/autolab_cycle_20261117_v143/REPLAY.py` from its root. This verifies the source/raw archives and independently checks complete signed weighted root sets without rerunning the original experiments or overwriting evidence.

DEPENDENCY_INPUTS.tar.xz.b64 additionally preserves the exact C139 certificate input consumed by the new applicability profile;DEPENDENCY_SHA256.json supplies its original-byte hash. Decode the base64 and xz tar into a fresh directory before any new frozen profile reproduction. The primary raw archive preserves old plans,attempts and generated evidence;do not extract it over existing frozen files. Use a new experiment ID/output filename and the normal hash/budget preregistration workflow for another execution. Absolute coordinator paths in old frozen manifests are historical provenance,not a reason to edit them.

PAPER_NOTE.tex is a standalone research-note source. The proof is a conditional finite-field component result. Its candidate-count bounds and correctness checks do not establish novelty,complete hard solves or the genuine matched end-to-end performance gate.
