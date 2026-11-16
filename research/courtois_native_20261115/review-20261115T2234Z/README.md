# Merge-resolution development review

Eight focused native tests, five compatibility tests, and the 256-vector Rust replay pass on crypto f2de3ffde441fd918786550ab0647b86e7bbcd2b / cryptanalysis 29093722c9e04f25e9f1dcb6d1a62a284e018f29. These are unsigned producer development checks; independent standing is pending. Raw logs, exact commands and source/input digests are retained here. This is not a performance measurement. [pcc:unverified]

The native test was repeated after the initial tool response was truncated so the shell exit status could be captured explicitly. Both raw logs are retained. The full root release suite was not repeated; its earlier 600-second build timeout remains the only post-input full-suite record. [pcc:unverified]

Crypto's automated merges retained every normalized journal payload but inserted conflict markers. The repair retains incoming main bb70178877d9ad51c1433309b5ebb3e33b0f33df's journal as an exact byte prefix, appends the original native entry, and then appends this review's entry. Full blocks and multiplicities from both merge histories are compared in journal-audit.json; only standalone marker/separator lines and whitespace are ignored by that comparison. Pre-existing main artifacts remain byte-for-byte. Frozen source, fixtures, prior evidence and crypto's canonical PCC ledger are unchanged. [pcc:unverified]

Cryptanalysis's automated merge rechained the already-published event for PCC-20261115-8d2645: seq 143 / hash 6eb355f2f236d8e3ecf23776a5e2ac0895a3c30d591e08e1b1b8a4abda3509b4 became seq 144 / hash 339345474c6ac548bcc894f1f0504637bddf2f80261c3a0fa818a7faef16cbfb. The claim body and frozen report files are unchanged. The main-parent prefix check reports chain ok, but the published-branch-parent check exits 1 (history rewritten); both outputs are retained in ledger-verifiers.json. Auto-merge for PR 1572 is disabled pending canonical PCC maintainers' authorized, provenance-preserving reconciliation. No ledger or claim is edited by this review. [pcc:unverified]

Current shared lab-contract content in crypto-autoresearcher is vault-encrypted; KEY is not configured here. Both readable repository AGENTS.md files explicitly prohibit ledger rewriting, so no inferred reconciliation exception is used.

Checkpoint upload access, complete pricing and an admitted native runner/image route remain open before measurements. No paid compute was launched.

Whole-file journal prose lint reports five inherited findings; output and exit status are identical on incoming main and the reviewed journal. The new review text and PR body lint without findings. The inherited entries are retained, with the comparison in inherited-journal-prose-lint.json. [pcc:unverified]
