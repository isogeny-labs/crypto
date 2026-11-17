# Exact committed fixture replay (independent language)

**Timestamp:** 2026-11-11 UTC. **Research status:** development self-check [pcc:unverified], not signed independent review.

Read the **exact bytes** of `sextic_twist_3isogeny_vectors.json` by GitHub blob SHA `11983c37a5692064435a4d4d42a2829f4fabe5bb` from commit `31bf54e8f93631aeb8b937a54bef0ffea181b6f8` (same branch). Parsed JSON and replayed in a fresh JavaScript V8 isolate using newly written elliptic point addition, double-and-add and Euclidean modular inversion (separate from the Python/Fermat-power scratch oracle that generated the vectors). Checked:

- **17/17** table rows: source is `[k](28,21)`; target is `η([4k](28,21))`; endpoint lies on `E_4`.
- Enumerated both full rational point sets independently: `#E_2(F101)=#E_4(F101)=102`.
- **10,404/10,404** ordered pairs (including infinity): `η(P+Q)=η(P)+η(Q)`.
- No exception, mismatch or unhandled point was observed. The rational three-isogeny formula maps x=0 to infinity; the same numerical formulas were used to replay the checked table.

This **does not** verify the full `g_* M f^*` implementation on arbitrary Jacobian divisor classes, the geometric identity with formal proof assistant, or a full cryptanalytic end-to-end cost comparison. It does verify that the machine-readable artifact actually committed to GitHub matches the reference mathematical isogeny on the entire source elliptic rational group.

Inputs were public synthetic small-field data, no cloud job, no checkpoint bypass or spend.
