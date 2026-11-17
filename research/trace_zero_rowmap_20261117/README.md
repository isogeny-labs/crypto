# Packed S3 row evaluation for ECC2K-130 index calculus

Implements four-bit combination tables for the existing projected S3 coefficient/RHS maps. The first16 rows are packed; wider systems/later rows retain scalar parity. Same equations, row/model order, numeric factor-base/kernel admission, complete W check, roots and original-group replay.

Twenty-one focused release checks pass, including65-case exact baseline/candidate parity, all basis bits/random packed-map inputs, widths>16 fallback and forced late scalar rows. Current field/context code is used in both arms. Existing malformed main SAT code requires pinned nonexecuted SAT/chain support for this focused build; broad root compilation separately fails tensor metrics imports [pcc:unverified].

[Protocol narrative](PROTOCOL.v2.md), [canonical protocol](protocol.json), [source hashes](SOURCES.sha256), [focused checks](checks/validation-tests.txt). The90-row controlled comparison has not started. The source/protocol will be pushed before calibration/pilot; final freeze precedes primary data.

The new comparison is against the merged optimized kernel, with lazy context fixed and per-query tables charged. No performance conclusion yet. Earlier studies/pins are unchanged. This work is only the algebraic relation-finding component of index calculus.
