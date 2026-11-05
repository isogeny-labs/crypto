# Independent v3 intake delta review

Read-only review on 2026-10-30. Combined patch 2baef8b8e4716bdc8abe7e288d395a5fa639d71b4a46d335472d6a61461a8266; CLI bd6812ade26247363a513503c106ee6d5e6ee58f5489b58201873560c592a666; native resource implementation 9b69036035fa6c4f8c386265655e9883f46eddeb60c0c441cefae02a29fd4b6c. Rehashed these files. The native implementation is unchanged from the reviewed v2 temporal correction; this note reviews the CLI addition, not a fresh execution of earlier fixtures.

No concrete blocking defect found within the explicitly caller-authenticated intake contract.

The CLI compares exact collection bytes with an externally supplied digest before interpreting them, rejects duplicate collection keys, checks collection schema/status, request, attempt zero, expected nonempty Pod UID and exact owner bytes. Native intake then validates request/binary/session/journal/preparation bindings, duplicates, clocks and intervals. Wrong owner bytes, Pod UID or request cannot be substituted while retaining these pins. Expected binary is additionally matched against the owner post-exec observation and journal tails. Runtime-image verification remains the authenticated controller placement chain, not a newly checked CLI argument; the request digest pins that requested runtime context externally.

The supplied collection digest, request digest, binary digest and Pod UID must be authenticated independently. A caller can of course hash a forged document and supply its own expected values; the program explicitly does not claim to prevent that or authenticate AWS. The existing controller image/UID checks are necessary provenance, not replaced by the new content checks. This is a reproducible integrity/association verifier under that trust boundary.

Input files use bounded reads, regular-file checks and Unix O_NOFOLLOW/O_NONBLOCK. O_NOFOLLOW protects the leaf, not symlinked ancestor directories. No untrusted content selects another path and all consumed bytes are bound downstream, so this is not a full directory-confinement API. Non-Unix nofollow support is not claimed here. Caps are 256KiB for collection/owner/preparation and 8MiB for journal.

Success exposes the distinct enclosing wall endpoint and explicitly leaves exact runner exit and active worker time null, with owner-receipt publication pending. It does not sum the nested reap/sync offsets or qualify a mathematical result/timing. Failure prints unresolved/null and exits nonzero. Stdout publication is outside the reported endpoint. Attempt zero is a deliberate restriction; restored attempts are not silently admitted.

Two CLI fixtures are source-reviewed only (external digest mismatch; exact owner/Pod association). No compilation, test, mathematical execution, cloud request or artifact publication was performed. Cargo registration and software validation remain the integrating owner's work.
