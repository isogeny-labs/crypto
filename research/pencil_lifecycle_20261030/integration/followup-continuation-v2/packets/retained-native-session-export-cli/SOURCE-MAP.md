# Source and interface provenance

The CLI delegates postrun validation to the exact retained exporter checked by exporter3. Its source digest is 642995d64c365a83a6fe414129189f2046b26cb60e96715ba3c9a7f6b55af7f7. The included custody module is the original ecbench_owner_closure.rs plus the previously checked thin public exposure; no validator body was rewritten.

Native diagnostic_resources.rs lines430 onward declares FreshOwnerPins with serde deny_unknown_fields and the actual close_fresh_enclosing_evidence method. The CLI uses that actual strict type inside strict Files, Expected and Request types. diagnostic_binding_core::parse supplies duplicate-key rejection, and canonical::sha256_hex supplies exact byte digests. Input code calls the existing exporter, which invokes diagnostic_file_adapter::check_file and the typed validation path.

The new main.rs binds executable bytes through the fixed Linux /proc/self/exe link and request bytes through a bounded O_NOFOLLOW regular-file descriptor. This identifies software and comparison inputs; it does not authenticate their caller or external owner. All supplied custody remains an external trust obligation.

CLI fixture code is the prefix of the checked exporter3 controls before its tests, with Fixture visibility opened only to sibling test code. Eight new entrypoint tests invoke the actual CLI executable as a child; four inherited custody tests remain present but are explicitly filtered from this new entrypoint sequence. Their prior twelve-control receipt is preserved separately. No test uses a mirrored native acceptance implementation.
