# Exact archival owner source copy

The active owned module lives in the separate CryptoLean repository at commit93b303becebc0f18b819cad5b8da180047fc887a. This consumer evidence directory stores its immutable source003 text and raw accepted/failed outputs,not a second active library implementation.

Receipt049 also pins PolynomialFirstMoment.lean,the exact copied source used for provenance. That path is an ignored derived archival payload. Run `bash hydrate-owner-source.sh`:it copies source-003.lean.txt byte-for-byte only if absent,and refuses any existing byte mismatch. Both canonical source and source003 have the same SHA256. No receipt/claim/path/hash is changed. The local payload remains present and exact;the tracked raw text reconstructs it without cloud transport.

The owner target build and existing pinned module axiom auditor are the relevant checks for the owned code. Consumer prototypes import the owner .olean by explicit experimental LEAN_PATH overlay,without changing the old dependency manifest. This recipe does not supply full repository gate/fresh coverage or waive any validation. Cache/binary outputs are not tracked. [pcc:unverified]
