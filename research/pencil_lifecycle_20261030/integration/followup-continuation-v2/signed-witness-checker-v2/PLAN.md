# Checker v2 bounded validation

Task T-1408. Preserve original signed-witness packet. Bind original revision and input hashes. Verify exact unique archive/manifest/source/certificate/summary sets; reconstruct rows; call only pinned check_certificate.verify, then check terminal scalar, exhaustion status, classification and summary joins. No producer, Gaussian elimination, collector, build, cloud or scientific run.

One local process, 180 seconds wall deadline, 1 GiB virtual memory, fixed 64 retained streams, no input expansion or automatic retries. Run `timeout 180s bash -c 'ulimit -v 1048576; python3 -B check.py'`. Negative controls cover omitted/duplicate/unexpected coverage, summary changes, scalar/row changes and terminal changes with internally updated bindings so semantic checks must reject them. These are deterministic software checks, not a benchmark.

Original source reviewer authored this wrapper; its successful checks do not constitute independent review of the wrapper.
