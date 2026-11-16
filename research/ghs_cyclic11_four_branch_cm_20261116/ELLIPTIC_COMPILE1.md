# First elliptic-source compile attempt

The pre-run compile of `elliptic_sources.rs` reached Rust code generation
but failed to create its temporary directory under
`/Volumes/SSD990/llm/tmp`: `No space left on device (os error 28)`.
There was also one unused-function warning; the unused `sub` helper was
removed before the first research execution. No elliptic model was tested.
The next compile uses `TMPDIR=/tmp` on the system data volume. The
protocol and corrected source will be committed before the first run.
