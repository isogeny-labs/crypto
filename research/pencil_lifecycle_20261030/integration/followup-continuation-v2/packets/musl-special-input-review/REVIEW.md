# Additive special-file closure review

The completed exact-b985 tree inventory matches SHA256 01140e75482e40a0d7860c634c3b3bc365928db6e3b648e0e0c85d60400fe9d0. It records 58 symlinks and one gitlink. All 58 link payloads were read directly from Git blob objects and recorded in special-files.json. No symlink target was opened or followed.

None of these special paths lies beneath src, the seven local dependency directories (including optional five-leads), or docs. Their locations are agent skill aliases, historical benchmark/research artifacts, Lean dependency aliases and an ephemeral test AWS shim. The sole gitlink is verification/pqc-assay/upstream. The local packages have no build scripts, and their reviewed manifests do not declare these special paths as dependencies.

An exact-commit source grep over src and the active local dependency trees for the special-directory families found only an unrelated ic_validate binary benchmark string, an icprog autolab comment/runtime protocol string, and a pencil protocol README reference to agent skills. None is an ecbench/library build-time reference to a special file. Existing observed ecbench/library include paths remain within ordinary tracked sources/docs. This supports excluding all 59 special entries from the selected build input closure: do not hydrate pqc-assay or follow historical links merely because they occur in the complete tree.

This is a source-based closure review, not a dynamic file-access trace or an audit of arbitrary registry build scripts. Registry/compiler inputs retain their existing pins and qualification obligations. No checkout, build, submodule initialization, external target dereference or scientific execution occurred.

Actual invocation configuration supplied by root: HOME=/home/ec2-user; CARGO_HOME is unset, so Cargo's default is /home/ec2-user/.cargo. The previous review found no ancestor/home config/config.toml files. Reconcile the fresh source .cargo against the same exact tree at admission; no credential values were emitted.
