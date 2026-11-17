# Targeted Git object-preserving storage maintenance

Decision: ROUTINE_CONDITIONALLY_ADMITTED_AS_METADATA_ONLY. No pack/prune/gc or workingtree mutation executed by this reviewer. Read installed Git2.52 pack-objects/prune-packed help. User-authorized local checkpoint work permits reversible object-preserving Gitmaintenance; this does not discard research evidence or need a new approval step.

Use the curated stage repository only. Freeze selected OIDs and before/after logical object-set digest, selected object type/size/content hashes, refs/HEAD, index/reflogs and workingtree file hashes; prevent concurrent Gitwriters. First256smallest validated looseOIDs are a storage bootstrap, not reachability selection. Passing explicit OID list without --revs packs those objects even when unreachable. No --thin, --all filter, expiry or garbagecollection.

Exact central commands, run by parent after its preflight (OID list on stdin; replace HASH with actual pack-objects stdout):

    git pack-objects --threads=1 --window=10 --depth=10 --compression=9 .git/objects/pack/pack
    git verify-pack -v .git/objects/pack/pack-HASH.idx
    git prune-packed -n
    git prune-packed

Use canonical pack-HASH.pack/.idx names, not an arbitrary preserve-v9 prefix. Verify successful self-contained pack/index CRC/content and all selectedOIDs present BEFOREpruning, and create pack-HASH.keep sidecar to protect evenunreachable preservedobjects from futuremaintenance. Selected cat-file reads beforeprune might still use loosecopies, so they alone are insufficient: verify-pack must validate the actualnewpack. Afterprune repeat selectedcat-file native objectidentity/type/size checks and confirm every before-object is stillavailable, with identical logicalOIDsetdigest and unchanged refs/HEAD/index/reflogs/worktree.

prune-packed removes only redundant loose copies represented in packs; it does not remove the Gitobjects themselves or traverse reachability to discard history. It may remove OTHER already-packed loose duplicates globally as well as the selected256, so inspect the dryrun list and validate their containing packs instead of claiming it targets only256paths. No gc --prune, gitprune, repackdeletion, unreachable expiry or direct loose-file rm. Frozen source/input/attempt/archive bytes remain untouched; only physical Gitobject representation changes.

With224KiBfree, bound firstpack temporaryspace beforelaunch: selectedcompressedpayload pluspack/index/keep/temp overhead mustfit, withcompression9 and1thread. Do not assume every <=1KiBcompressed looseobject has <=1KiB decodeddata. If pack/verification fails, do not prune; retain originals and diagnose. First successful bootstrap frees block-padding while retaining allobjects; only then consider the remaining smallobjects with a fresh preflight. No scientific experiment, measured speedup or history-loss claim follows. Actual execution receipts must report before/after invariants and reclaimed bytes, not infer success from this staticreview.
