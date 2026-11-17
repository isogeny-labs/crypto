# Report custody

The self-contained validator report remains Git-tracked under the existing artifact policy: JSON below 1 MiB is not selected for migration. Its 171,195 bytes have SHA256 `c355a3414f75b2796d24ee9c48824de46e58c583d9bc4ab7212e651fc7630f6e`. Trusted publication run 43817399144 selected no data files, so no upload or remote replay is claimed.

The numeric-fidelity guard checks this committed report against the preserved original using exact decimal parsing. Original reports, prior binding manifests and stored objects are unchanged.
