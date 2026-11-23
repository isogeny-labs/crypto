# Exact log packaging

The original 43-file package manifest was verified after copying. Several
verbatim compiler/probe/gate logs contain line-end whitespace. They are stored
as timestamp-free `.log.gz` archives instead of changing their bytes. Their
original paths and raw hashes remain in `PACKAGE_SHA256.txt`; sources, proof
statements and verification outcomes are unchanged.

Run `bash verify-original-package.sh` here to verify every original manifest
entry, transparently reading archived logs. Alternatively decompress `.log.gz`
files in place before running the original plain `sha256sum -c` command.
The full raw logs remain in the audit workspace as well. This is a packaging
change, not a proof correction or a new build.
