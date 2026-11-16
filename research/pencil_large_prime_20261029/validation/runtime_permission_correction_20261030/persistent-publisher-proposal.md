# Future canonical publisher correction (proposal only)

Conductor returned Clear for `taskq/deploy/eks-karpenter/resume/publish-runtime`.
No canonical source file was edited and no new reservation was claimed.

The approved runtime archive contains the correct progress bytes and executable
mode, but extraction under the publisher's restrictive umask produced mode0700.
After archive and internal SHA256SUMS verification, explicitly set only
`$work/runtime/isolab-criu-progress` to mode0755. Preserve every file byte and
other member mode. Add a publisher regression fixture with umask077 asserting
that the Docker context contains progress0755 and unchanged content hashes.

The generated image tag must also include a stable recipe revision such as
`progress-mode0755-v1`. The existing tag only hashes base digest and archive hash;
reusing that formula after a mode correction would reuse the failed image's tag.
The correction's identity therefore needs the recipe revision in the tag input,
in addition to the existing archive and base identities. Resolve and record the
published digest normally; never silently treat the old digest as corrected.

Suggested minimal lines (not applied):

```bash
chmod 0755 "$work/runtime/isolab-criu-progress"
permission_recipe=progress-mode0755-v1
tag="recovery-$(printf '%s\n%s\n%s\n' "$base" "$hash" "$permission_recipe" | sha256sum | cut -c1-40)"
```

The first line belongs after internal archive hash verification, before Docker
build. The tag line replaces the current two-input tag calculation. This is a
future persistence change with its own source review; the current gate already
has a separately published and independently verified mode-corrected digest.
