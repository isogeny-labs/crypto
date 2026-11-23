# Original package preservation

The untouched20-file `source-manifest.sha256` binds the original payloads.
The verbatim `receipts/compiler-version.txt`, including its trailing blank
line, is packaged with timestamp-free gzip. Use
`bash ../verify-gzip-manifest.sh "$PWD" source-manifest.sha256` from here to
check all original bytes. Source, protocol, failed attempts and successful
outputs are unchanged. The top-level check helper performs this verification.
