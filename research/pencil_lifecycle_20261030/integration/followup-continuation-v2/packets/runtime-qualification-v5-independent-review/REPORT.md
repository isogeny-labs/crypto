# Independent fixed runtime qualification review

Accepted fixed software Q0-Q4 qualification and the recorded qualified_stopped disposition for owner 24806e07-13ac-4791-92da-12dd595ae9e4. Exact image 3a2bb98a and binary d0e2d142 bindings, full hashes in review.json, match the published admitted inputs.

Observation: Q0 audited the image and runtime files. Q1 exited 0 with the exact expected static preflight report. Q2 rejected the wrong request hash with exit 2. Q3 rejected the wrong owner digest with exit 125. Q4 checked unchanged fixture and gate contents. All three container writable-layer diffs were empty and all four owned containers were removed.

Descriptor evidence: each tested exec has a possible-publication record followed by a confirmed-publication record. Root-owned mode 0444 token hashes match the exact nonces. Host PID and cgroup identity match gate PID 1, UID/GID 1000, pids.max 1 and pids.current 1 observations before the tested exec. Container inspection retains readonly root, network none, drop ALL, no-new-privileges, private cgroup and configured resource limits.

Measured costs: inner qualifier 6.383614638 seconds; complete outer attempt 39.186950947 seconds. Reviewer extracted 81 selected files in 2.793226771 seconds using read-only debugfs, with 91,635 bytes stdout/stderr and unchanged disk stat. No mount, daemon or runtime rerun occurred. The completed worker supplied current disk hash f38930c6; reviewer did not rehash the 4 GiB image.

Disposition: native invocation and completed-unit success join the final qualified_stopped receipt. Retained observations show absent cgroup, detached loop and stable disk custody. Previous temporary-directory and read-only-root transport failures remain preserved with unchanged charges.

Scope: PID enforcement was observed after startup and before each tested exec, not earlier or continuously. Q1 remains static_inventory_only, external owner authentication required and scientific execution unadmitted. This completes the fixed runtime software component; authenticated owner custody, prospective protocol, timing qualification, prior-p739 applicability and exact cohort admission remain separate obligations. Publish these evidence records before relying on the qualification downstream.
