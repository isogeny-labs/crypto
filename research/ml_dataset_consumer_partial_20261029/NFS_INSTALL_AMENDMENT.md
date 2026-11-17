# Exclusive staged installation on the migration NFS filesystem

The first source-frozen CLI replay at e430ad68789191ec81b58cd4cc9b8bc70c3db540
failed on the unchanged retained P01 input with `renameat2(RENAME_NOREPLACE)`
returning EINVAL. Stdout was empty and the incomplete marker plus original
metadata/staged bytes remain in the failure packet. The migration home is an
NFS4 mount; unit-test temporary roots are tmpfs. The frozen release suite, strict
Clippy and 42 sealed byte comparisons passed, but that does not close the failed
new CLI gate.

Linux documents that RENAME_NOREPLACE requires filesystem support, and EINVAL
can indicate unsupported flags. See [rename(2)](https://man7.org/linux/man-pages/man2/renameat2.2.html).
The bounded correction applies only to ENOSYS/EINVAL/EOPNOTSUPP: descriptor-
relative linkat installs a new name without replacing an existing destination,
flushes its containing directory, then unlinks the owned staged name and flushes
both containing directories. Existing destination names always fail; an ordinary
overwriting rename is never used. [link(2)](https://man7.org/linux/man-pages/man2/link.2.html)
specifies no replacement and warns that NFS may perform a link before reporting
an error. Therefore any error returns failure, without inferred rollback or
automatic adoption. The tested pre-removal failures retain the incomplete marker;
a late directory-sync error after marker removal can leave it absent without
returning success.

The source stage must be one regular, non-symlink file with exactly one link. Its
opened descriptor remains live. After linking, both old and new names must still
resolve without symlink following to that same device/inode with exactly two
links before removing any staged name. A deterministic source replacement after
the initial open rejects instead of adopting another inode. During successful
link installation it temporarily has two names. Final source and artifact audit still
require single-link regular files after stage removal. The incomplete marker
protects the multi-syscall transition; a fault after linking or before unlinking
retains both names and fails. Completion still requires the fresh exact inventory
and all directory flushes. This preserves exclusive atomic installation of the
new complete file; stage-name removal is a subsequent operation.

This records checked syscall ordering and fresh local hash/audit behavior. It
does not establish server power-loss or crash durability, and it does not claim
an atomic transaction against an adversary continuously rewriting a directory
owned by the restoring process. No ambiguous failure is converted into success.

Before final acceptance the amended source requires forced-unsupported branch
positive/existing-name/fault tests, a new source-bound full standalone suite and
strict Clippy, and fresh local CLI output on the actual NFS destination. The old
failure and prior passing evidence are retained unchanged. No producer, real
remote call, dataset campaign, IAM change or budget extension is authorized or
performed by this correction.
