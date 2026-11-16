# R133 handoff

- Lambda derivative is zero only at the P11 linear base root `1242` and the P22 linear base root `1612991`.
- Four other base places have derivative units: P11 residue degrees 11,55; P22 residue degrees 44,88.
- Run `python3 research/koblitz_modular_ic_20261018/rounds/133/derivative133.py research/koblitz_modular_ic_20261018/rounds/133` and `python3 research/koblitz_modular_ic_20261018/rounds/133/verify133.py`.
- Result SHA-256 `2e5f7e397d11f867a5aacc858d714f7a79a1521fa6a142ae0a2efe235152f2a2`; byte replay and corrected changed-source control passed. Lean degree-partition file passes.
- Next: factor full G51 at four ordinary base places and normalize at the two exceptional linear places. Evaluate N51 only after normalized lambda places are certified.
