# Publication-only custody correction
The frozen original validator incorrectly assumed all60source pins were
Git blobs. One is the raw R34 proof-cache manifest. The new checker verifies
59direct historical Git blobs and authenticates that one exact donor hash
through R34's committed runs.sha256 at the sealed source revision. It keeps
all original terminal-receipt and fresh-download checks. The first failure
is retained. No scientific input, result, source pin or archive is modified.
