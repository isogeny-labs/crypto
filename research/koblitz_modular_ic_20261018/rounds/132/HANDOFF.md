# R132 handoff

- Exact base fibers at R100 P11 and P22: three places each, residue degrees `(11, 11, 55)` and `(22, 44, 88)` over F2; all base ramification indices one.
- Producer: `python3 research/koblitz_modular_ic_20261018/rounds/132/base132.py research/koblitz_modular_ic_20261018/rounds/132`.
- Independent check: `python3 research/koblitz_modular_ic_20261018/rounds/132/verify132.py`; Lean: `lean research/koblitz_modular_ic_20261018/rounds/132/lean/BaseFibers132.lean`.
- Result SHA-256 `8848edb1a7f1abfed1070260fd013d3e8194ba7bb998f936f7a6bbdb3667a43c`. Byte replay passed; changed P11 input rejected before output.
- Next: normalize the degree-16 lambda extension above these six base places using the actual discriminant branches; then evaluate N51 at every lifted place. Keep infinity separate. Do not infer full degree-112 fibers from the base census.
