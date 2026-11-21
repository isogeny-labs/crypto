# Source replay for frozen supervisor and registry patches

Run `python3 check_patches.py --repo /path/to/crypto --packet /path/to/authentic-supervisor-registry-preparation-20261107 --receipt /new/path/receipt.json`.

The repository must contain commit3517ae554787690472e3ed9801623de27730e20d and rustfmt+1.98 must already be installed. The checker reads base files from that Git object, checks frozen patch digests, applies both patches in a temporary directory, compares assembled hashes, and syntax-parses six Rust files. It does not compile or execute solvers, change the checkout, or overwrite receipts. Use trusted archived packet hashes; this tool is not a signature/authenticity authority.

REPLAY.json records the successful fresh check. NEGATIVE_CHECKS.json records refusal of altered patch bytes and altered assembled-source hash. Neither test establishes Rust type correctness, original-cell replay, CLI operation or scientific dataset production.
