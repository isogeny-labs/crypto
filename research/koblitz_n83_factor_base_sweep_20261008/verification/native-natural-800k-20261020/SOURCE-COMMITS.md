# Exact local source commits retained for connector publication

The migrated host has no Git CLI write credentials. The authorized connected
GitHub app can publish Git blobs/trees/commits, but its commit API does not expose
original author/committer timestamps. The published snapshot therefore uses a
new connector commit with the same final tree rather than rewriting the recorded
experiment source IDs. Existing remote history remains its parent; no force update.

SOURCE-COMMITS.bundle retains every exact local commit since prior PR head
f0947816c8e474d89c20cebc340e3c71e65a8da9 through source-bundle-tip.txt, including
all frozen v12/v13/v14 source commits and the completed report/validation commit.
Its own enclosing Git commit is deliberately outside the bundle to avoid recursion.
Bundle SHA256, heads and successful git-bundle verification receipts are adjacent.
The only prerequisite commit is the prior PR head, already in repository history.

From a clone containing that prerequisite:

    git bundle verify research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-800k-20261020/SOURCE-COMMITS.bundle
    git fetch research/koblitz_n83_factor_base_sweep_20261008/verification/native-natural-800k-20261020/SOURCE-COMMITS.bundle refs/heads/codex/n83-resume-20261018:refs/heads/n83-local-source-800k

The recorded source786a8af0f7f935f3e17e9be21b8930eaad113c61 and earlier local
freezes can then be inspected by their exact IDs. The raw archive separately
preserves full relevant source archives, exact measured ELFs and all actual
attempts. This Git bundle is source-history custody, not a new measurement or
successful S3 publication. Connector upload verifies every blob SHA and final
tree SHA before its leased fast-forward branch update.
