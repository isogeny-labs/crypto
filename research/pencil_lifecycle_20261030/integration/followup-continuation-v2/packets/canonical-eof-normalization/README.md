# Canonical terminal-newline normalization

The two normalized modules remove only surplus LF bytes at EOF, leaving exactly one final newline. Original exact reviewed sources are retained alongside the normalized files, and original packets are unchanged. `normalization.json` binds both versions and checks byte equality modulo this precise transform. Every dependency listed in each original snapshot was hash-checked before compilation.

Each normalized module was freshly compiled against the same retained dependency search paths with the pinned Lean binary, one thread, 4096 MiB and a 120-second timeout. Native stdout, stderr, commands, exit codes and tool hashes are retained. This is formatting normalization, not a new theorem or independent semantic review. Root integration is separate.
