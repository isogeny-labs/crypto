# Complete raw publication receipt

The existing trusted-base artifact-autofix43319185494/job131535060041
completed successfully on5fcf. Its whole job log explicitly records two
remote objects replayed before the normal bot push. Exact bot commit
5bec81e36ac9b6f7b91ddc5437766248bf7aacfe,parent5fcf,adds the canonical
manifest/ignore and removes only the two original whole archives. Local
checkout is fast-forwarded to that exact commit. [pcc:unverified]

canonical-manifest.json is an exact copy of the bot-produced manifest.
custody.json links immutable canonical Git and content-addressed S3 paths,
all original bytes/hashes,whole UTF8 log/raw+archiveSHA256/standardXZ/exact
roundtrip,and actual artifact/wiring checks0. The original whole archives
remain intact in scratch;no tree was split or pruned. The earlier no-op
job3eeb2e is not used as upload proof. [pcc:unverified]

This additive receipt supersedes only the raw-publication-pending status
in immutable original400f38/4a80d5 reports and custody. Full01 remains a
failed timeout,NOT accepted;full02 remains complete local acceptance.
No scientific code,counts,claims,old reports,criteria or CI jobs change.
The consumer will link this canonical manifest without creating duplicate
consumer artifact manifests for files that are not CI runtime inputs.
[pcc:unverified]
