# Pending actual-root integration checks

This package deliberately links actual crypto_lib, not a shim. Its tests target the adapter in ../pending-actual-root-integration.patch; that patch is not applied to live runtime files because the currentroot baseline cannot compile. The intended36case matrix covers both declared fields,n3/4/6,two wireforms,three kernels,baseline basis equality and publicmutation rejection. These tests have not passed; no full-root integration claim is made. Repair baseline source corruption and review/apply the patch before running cargo test here. [pcc:unverified]

After later source recovery, the small dependency repairs in the patch are already applied; apply only its certified.rs hunk. The actual-root test matrix is still unrun because109 other baseline API/type diagnostics remain. See ROOT_RECOVERY_REPORT.md.
