# Reject hard-linked copied mutations before writes

Frozen before edits. Parent source review found that canonical directory
separation alone does not exclude hard-linked artifact aliases. Current01 checks
original content after mutation, which detects damage too late if a copied edit
shares the original inode. The standard copy_tree creates fresh files, but the
helper must enforce its own exclusive-copy precondition at every mutation path.

Preserve01 and candidate07 as unexecuted history. Add a native MetadataExt nlink
check requiring exactly one link for every edited/resealed/removed copied file
before the first write. Add a hard-linked factor-base regression proving failure
before any original or copied bytes change. Keep all previous five tests/recipes,
limits and no-semantic-verdict behavior. No production workflow activation.

The helper assumes the caller owns the copied namespace exclusively, as already
required by01. This does not claim protection against concurrent unauthorized
filesystem replacement. Input source identity/readback and global workflow caps
remain separate requirements. New controller candidate08 has unchanged library
sources relative to passing06; source/body guards remain mandatory.
