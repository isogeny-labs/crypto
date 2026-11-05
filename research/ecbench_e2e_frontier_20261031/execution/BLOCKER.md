# Execution blocker (2026-10-31 UTC)

The isolated run is prepared but was not submitted from this workspace.

## Required route

Repository policy requires x86 CPU performance runs to use Iso Lab EKS Spot,
with a committed protocol, exact source/binary identity, L2 admission, an A/A
control, replay-all, and an S3 checkpoint/result bundle.  A shared local cloud
container is not a valid wall-time substitute.

## Observed blocker

- `conductor`: command not found;
- `aws`: command not found and no configured AWS identity;
- no Iso Lab/EKS connector is exposed to this session;
- `gh auth status`: the configured `GH_TOKEN` is invalid;
- no callable GitHub connector is installed;
- the environment exposes no relevant secret or outbound identity.

The public egress IP is `90.96.183.30`, but allowlisting an IP alone does not
provide AWS or GitHub authentication.

## Local preparation that did complete

- release build from source commit
  `48c8a4767d2d1eed6a7b379f74410bc729239380`;
- binary SHA-256
  `91f5895637501c6de257bbaf3afc9cbc2c0f5e964964d3dc4fc3bf78a427d96b`;
- `ecbench methods --json`: 74 methods;
- host capsule `ECBENV3h0bb9eb2ab583` (AMD EPYC 9V74, five visible physical
  cores, KVM); this capsule is for preflight provenance only;
- campaign specs and protocol validation.

## Unblock condition

Provide this workspace an authenticated Iso Lab EKS submission route and a
valid repository publication route.  The executor then runs the immutable
request manifest, downloads and hashes the S3 result, replays all records, and
updates the ranked frontier.  No benchmark-design choice remains for the
executor.
