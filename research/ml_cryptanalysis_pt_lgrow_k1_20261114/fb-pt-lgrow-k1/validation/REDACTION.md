# Redaction of model identifiers (2026-11-14)

Repository policy forbids model identifiers in committed artifacts. The validator's files named the model it ran on.
Every match of the pattern `claude-(opus|sonnet|haiku|fable)-\d+(-\d+)*` was replaced by the literal text "model id redacted per repository policy".
No other byte was changed, so no number, check or verdict changed. The sha256 pins in `../VALIDATION.json` were updated to the new file hashes.

| file | matches | sha256 before | sha256 after |
|---|---|---|---|
| `validation/VALIDATOR.md` | 2 | `361a950a5b7f1b8bc51e2d00927eec15897ecb44bb3b79c53a5650ca1aa8c28d` | `7b65110684d9561ebe84a223c56b2a3a69f7b840a420bbb5c082b503abe2c938` |
| `validation/validator-report.json` | 4 | `cb1a6d5d6a4629b29de1cabbff0bc393c48cda9ac2422bb6add9e3e50dbac493` | `e98bdaf534293f80e3e7be89b0526a2ef96dd7035faaa910d4a7f38579a8b980` |
