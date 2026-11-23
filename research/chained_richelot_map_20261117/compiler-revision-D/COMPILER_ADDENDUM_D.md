# Isolated D compiler dependency and retained-type completion

Root77d and caller98ac are immutable. The first main compilation failed
before arithmetic because the full retained unit12 shifted source embeds
`fixtures-12.csv`, even though that old test is filtered. D copies the entire
original591-byte file unchanged. No fixture is selected from an arithmetic
outcome, and the original unit12 test population is not executed.
[pcc:unverified]

Two original imports are used only by the excluded old test populations.
Preserve both original prefixes byte-for-byte. The new model-assert helper
qualifies its argument as `super::Balanced`, and an anonymous compile-time
constant binds the retained `construct_for` function's exact signature.
Neither change introduces a constructor invocation or any runtime arithmetic.
No unused-import allowance is added; strict warnings remain errors.
[pcc:unverified]

All original method bodies, the runtime map/homomorphism/dual/conjugacy
assertions, fixed witnesses/scalars/controls/signs, one worker120s and
8MiB RAM/3MiB scratch floors remain as PROTOCOL_14. The new caller accounts
for the extra directory level and selects its own D source lock. The full
C negative is publicab9297b229b406a10581b85a176266c90c2da12c; the full B
negative is publicac01a90b6ee997fcf9ea99977d5d4b1d07a5ab1c.
First scientific arithmetic remains unexecuted when D is frozen.
[pcc:unverified]
