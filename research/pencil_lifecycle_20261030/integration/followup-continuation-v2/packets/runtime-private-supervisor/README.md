# Private runtime source packet

Read REPORT.md/report.pdf and INTERFACE.md. supervise.py implements separate private assembly and qualification phases; both binding drafts have admitted=false. They are reviewable inputs, not commands authorized to run. Missing admission/review/qualification predecessor pins remain null intentionally.

Software-only replay: `timeout -k 5 60 python3 run_tests.py` with a512MiB address-space limit. It replaces every external command method with a rejecting mock and uses temporary directories for file-mode/source-copy controls. Initial source/test results and the intermediate fixture failure are preserved with initial- and mode-fixture-failure- prefixes. AMENDMENT.md records the revisions.

No runtime path has been exercised. Main execution modes are --launch, --worker (inside the private service only), and read-only --reconcile. Root operator authorization plus exact reviewed admission/source/input joins are required before launch; merely editing a JSON boolean supplies no authorization. Any unresolved prior phase must be reconciled before a distinct phase is admitted. Runtime qualification, prospective review and campaign accounting remain coordinator responsibilities.
