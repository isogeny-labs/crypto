# V8 cache and root test binary custody

Artifact preservation only. The current cache inventory contains2,261 regular files totaling2,668,933,914 bytes,301 changed/added versus the preserved2,165-file donor inventory and no missing donor files. Full content/mode inventory is retained.

The validated delta archive has300 regular members and excludes the root test binary to avoid duplicating its253,684,264 bytes. A separate one-member archive preserves that binary, mode0700 and SHA e641bcf1d5e2be3369b52ca60ee0721fc8a89b734375e5334b026c31389feb88. Every archive member hash and mode was checked, then every source cache file was rehashed unchanged. All4,485 source files matched the exact v8 source manifest. Compiler JSON binds this root test executable and exit0; overall v8workflow remains failed at compile-driver. No test success is inferred.

The first archive was rejected because tarfile encoded an existing hardlink alias. It remains as cache-delta-hardlink-rejected.tar.gz with capture-failure.json; corrected archive dereferences regular bytes. No source cache or donor was changed, deleted or uploaded. Root owns publication and any later reclamation.

Capture script enforces a300-second local artifact-processing bound,2GiB uncompressed archive limit and free-space checks; these are not scientific execution allowances. Saved statvfs free-space observations are host readbacks, not guaranteed allocated capacity. Exact compiler/source/materialization/admission receipts are retained as binding files.
