# Preserved setup error

The initial setup command ran with the new lean-cosets-v1 directory as cwd,
but used paths beginning research/curve25519_torus_native_20261115/.
Both cp operations failed with "cannot stat ... No such file or directory".
The output redirection also failed because ../many-cosets-v1/development
did not yet exist, so no Lean process started.

The command had created only nested empty directories and a .lake/packages
symlink under lean-cosets-v1/research/. Inspected that tree, removed exactly
that symlink with unlink and its empty directories with rmdir, then ran
the setup from the repository root. No existing file or evidence was deleted.
Both actual Lean build attempts are retained separately; neither failed.
