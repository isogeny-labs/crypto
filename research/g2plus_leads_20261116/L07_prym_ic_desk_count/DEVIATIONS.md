# DEVIATIONS (all `[pcc:unverified]`)

D1. PROTOCOL command `python3 -I -m unittest -v src/test_l07.py` fails (exit 1, ModuleNotFoundError: No module named 'src'; `-I` removes the cwd from sys.path so the path-to-module conversion cannot import `src`). Output kept in raw/tests_spec_command_failed.txt. Minimal fix used: `python3 -I src/test_l07.py -v 2> raw/tests.txt` (same tests, exit 0, ends OK). Thresholds, arms, seeds unchanged.

D2. Added helper scripts not in SPEC, analysis only, no research choices: src/gates.py (evaluates PROTOCOL section 7 gates from raw outputs; raw/gates.txt) and src/plot.py (plot.svg).

D3. SPEC test 7 is garbled in the frozen text. Implemented as: x^2 - 1 mod 7 has roots {1,6}; x^2+1 at x=10 mod 101 is 0. In the unit test the MC test (test 8/9) uses T=20000 instead of 200000 for speed; the frozen-size cells are checked in gates.py (gate 3 and gate 5 on the real 200000-trial output).

D4. SPEC part B emits a Part-A d range containing d=1, for which g=1 hyperelliptic exponent is 0/1 in the TABLE (formula as given: 2 - 2/1). No change made.

D5. DERIVATION.md was not assembled: the task text restricts the executor to RESULTS.md with numbers only and no verdict. The frozen verdict (PROTOCOL section 8) is the director's to state.
