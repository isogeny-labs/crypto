This compact packet preserves scripts, corrected proposal, original failure,
positive/negative receipts and staged output hashes. Full source scratch and
fixture copies remain in the sibling parent preflight directory. They are
omitted from this compact review packet because the exact original inputs are
already bound by SOURCE_BINDINGS.json and the existing source recovery packet.

To repeat checks, use a fresh external directory and the retained trusted input
anchors. preflight.sh requires the 12 exact source paths in SOURCE_BINDINGS.json.
Run the original proposal once to retain its expected registration failure and
bound snapshots; run corrected-proposal separately; then test_preflight.sh builds
only relocated synthetic fixtures. Never compile or run a solver as part of this
source preflight. Restore/bind missing original files before attempting replay;
do not replace them with current files or treat source absence as an apply result.

All receipt scratch paths identify the original execution location; receipt
bytes are preserved exactly rather than rewritten to this compact packet path.
