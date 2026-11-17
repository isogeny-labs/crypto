# Replacement runtime contract

contract.json and qualification-binding.draft.json remain unadmitted. PLAN.md states proposed bounded steps and unresolved bindings. check_inputs.py reads local artifacts only; observe.py issues only bounded local image-inspect and container-list commands. Neither is an assembler or qualifier. Run the helper checks with `python3 test_checks.py` from this directory.

input-check.json lists 63 verified file bindings; local-observation.json retains exact commands and outputs. Initial checker/readback are preserved. REPORT.md, report.pdf and flow.svg communicate the evidence boundary. No prior admission is transferred to a new image or retry. Missing local artifacts cause verification to fail rather than recreate or replace them.
