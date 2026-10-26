# Validation status

In progress. Do not read planned checks as completed. The experiment panel
in PROTOCOL_DRAFT.md has not run. A final native-input binding and actual
command outcomes will replace this paragraph before publication.

Setup observations retained outside the source tree:

- Conductor returned127 before editing; unavailable, not passed.
- The initial scoped formatting command used unsupported `cargo fmt --bin`;
  corrected to rustfmt on the exact touched source files.
- The initial CLI test command encountered an off-cone tracked
  `isogeny_algos/Cargo.toml` path dependency. It was restored unchanged.
- Initial source materialization refused the not-yet-indexed new module;
  staging the module made it visible to the unchanged tracked-source helper.
- The checkpoint gate returned2 before upload (missing resume file); no
  new research run, S3 operation or credential inspection followed.
- The atlas --match command initially lacked its sparse orchestration module;
  the original failure was retained before restoring that tracked module.
