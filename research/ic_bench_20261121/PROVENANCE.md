# Provenance

Source: isogeny-labs/lfd-summation-systems, branch `claude/ic-bench-harness`
(PR #43, closed after this move), head `877ea5208219deec2426dd232dfe2754c50d1250`.
The PR's base on `main` was `2dc4a56719bb4c6d504fd1d93204e5b42637dfb6`.

The following were copied byte for byte from that commit, keeping their relative
paths: `csd/bench/`, `scripts/bench.py`, `tests/test_bench.py`,
`experiments/ic-bench/` and `experiments/ic-rootfind/`.

Vendored solver modules. Nothing outside `csd/bench/` was changed by PR #43, so
these match lfd-summation-systems `main` at the base above:

| file | sha256 |
|---|---|
| csd/__init__.py | ec09b2a4fa151ee1787fe55e64753ff23986ea37f049438587d470f95d5051f9 |
| csd/ic_dreg.py | 0e5a650d3717ac029e3cbbc10c8e7f1525c11f46461d651187ce40d89c25afe1 |
| csd/ic_rootfind.py | ef0335bf39e1c54ce68292a2c471e6e585bd16ace0d09a1ab27c80490da2acc7 |
| csd/_rootfind.c | 7b09ef3b21f2489699008d6086dce15e3632f6f4824ccee32d02c7365fc80fee |
| csd/elim.py | a490fdaf88033cb5b430a90f5b8e16bbac6549999c2abd13d8e1f7279f4d08e0 |
| csd/_elim.c | 1269131384fc5f110b90b70a1c8f174a60ca7fe968388f6e13f96eba56959290 |
| csd/gf2_ghs.py | bd0e24d087eab7c9119366c2f7ae0e47d8928bf7f66d32376cf04cba9e650271 |
| csd/field.py | af3f1d2c22f76396d92bef3b4f1824c88f4706d07ae1f92b0335c12f739160c3 |
| scripts/run_ic_rootfind.py | 1c964e398e0fb29bb48703997a060e5cf82de3b6b8556bfaf88ffc97b9477963 |

What was not brought over: the Rust kernel `rust/ic_rootfind`, which covers
n <= 191. The `rust-records` fixture set only reads its stored records, and the
C kernel covers n <= 63. Other `csd/` modules were also left behind because the
harness does not import them.

Checked here: `python3 -m pytest -q tests` gives 29 passed,
`scripts/bench.py fixtures verify` reports the fixtures verified, and a smoke
run with py_ref, enum and lrf finished with no mismatches.
