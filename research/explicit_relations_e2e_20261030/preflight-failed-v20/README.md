# V20 preflight compilation failure

The release driver built, and field/hash/filter host fixtures passed. Cargo test
then rejected an existing direct solve_reduction call missing its seventh argument.
No GPU experiment cell ran. The final archive hash was verified and pod deletion
confirmed. The full failed logs are retained in the content-hashed archive named
in failure.json; compiler-error.txt preserves the relevant error verbatim.

The required fix passes default20 to the old profile-equivalence test. A new
source/protocol version must pass the entire preflight before collecting data.
