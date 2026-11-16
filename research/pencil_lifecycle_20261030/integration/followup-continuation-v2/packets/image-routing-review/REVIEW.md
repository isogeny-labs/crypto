# Routing addendum: scoped follow-up review

The additive routing contract addresses the prior review's ambiguity: it names the local daemon legacy builder, sets DOCKER_BUILDKIT=0, uses an explicit local socket, and uses an allowlisted client environment with fresh empty configuration. Builder/context/TLS/proxy/credential selectors are not inherited. Unsupported legacy routing stops without fallback.

The addendum does not claim this route is available: client/server support, local upstream-parent resolution, immutable local image-ID FROM acceptance, actual final binary/source bindings and the bounded supervisor remain explicit gates. This correctly separates a prospective route from an observed one. --network=none and --pull=false are expressly not treated as registry-exclusion proof. The600-second shared cap and exact-state reconciliation remain unchanged. Producer hashes verified; no Docker/native/cloud action performed. Original proposal and first review remain immutable.
