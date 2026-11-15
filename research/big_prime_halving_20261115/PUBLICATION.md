# Arbitrary-precision point-halving continuation

Source review and protocol precede new implementation/calls. The parent
generated-setup component is merged in main230dfbc408 at final head2da6eb137f;
validation run43725611298, size43725611269 and artifacts43725611200 pass.
The latest PCC run43725611409 passes; the earlier same-head PCC run was
superseded/cancelled by concurrency, not counted as a pass. The unrelated
records check remains the existing open benchmark record. [pcc:unverified]

Critical/cover PR4304 is now based on main with auto-merge. Its journal is
dirty against the new parent handoff and has one Copilot request; preserve
that file while pending. This independent worktree starts at4b8e8797b8 to
inherit the pending PCC prefix, and uses this publication record meanwhile.
No native research code or qualification has been written/run for this port.

Sources: parent sampler/tower/reference and SETUP-BOUND; actual upstream FLINT
v3.1.3 primality, square-root and root code plus installed declarations;
MIT18.783 Lectures2--5, especially Lecture5 Theorem5.13; the existing arbitrary-
precision guarded-DAG controls, whose interface differs from curve setup.
KB unavailable. Current online FLINT3.7-generated implementation prose is
not a statement about the installed3.1.3 package. Primary hashes are retained.

The repository curve-algebra workflow is applied for field/map/subgroup/cost
boundaries; its references and scope lessons have been read. Its unrelated
CM fixtures are not cited or rerun. The proposed general ring identities are
to be built in the shared library before numerical qualification; they give
no native-program or group-homomorphism proof on their own.

Workflow correction: validation-gate has an unfiltered pull_request trigger;
size/artifacts are main-only. Earlier continuation notes loosely grouped all
three as main-only. Only the latter two wait for the main retargeting. The
conflicted parent's target workflows ran while ordinary PR workflows were
absent; merging main restored its required head checks. [pcc:unverified]
