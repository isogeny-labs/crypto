# Review addendum

Verified the final replay_capture.py differs from the previously reviewed hash only by explicit work_scope and allocation_scope fields and exposing the existing reader retention summary in wire-mode output. Removing exactly those three additions reconstructs the previous file byte-for-byte and matches its saved SHA256. The previous reporting precision issue is resolved: row replay allocations are explicitly distinguished from selected-decoding retention. No mathematical admission, lineage, budget arithmetic or replay control flow changed.

Final replay_capture.py SHA256: `67edd59b9fea4a82e1a91e7282b3811ca7a344d3270a38d7514e42851be4e41d`.
Previous reviewed SHA256: `e281e20be166e253599560a803d8376990f1c0e0a885582c5e66e984e30d1660`.

This addendum is a read-only exact-source/diff review, not a new test or full-source typecheck result. The root agent's separately reported test/typecheck results were not rerun here. The original frozen review packet remains unchanged.
