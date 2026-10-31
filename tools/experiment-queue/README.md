# Experiment queue controller

Native Go orchestration for [the scheduled EKS queue](../../docs/experiment-queue/README.md).
The controller validates reviewed experiment metadata, makes permanent execution
claims, submits through the existing Isolab CodeBuild bridge, checks returned
observations, and opens result PRs. Cryptographic work runs in the task image on
EKS, not in this program or on the Actions runner.

```sh
go build -o experiment-queue .
go test -race ./...
go vet ./...
bash -n worker.sh remote.sh
```

The contract tests require `EXPSPEC_BIN` pointing to a build of `../expspec`,
`EXPERIMENT_SCHEMAS` pointing to the canonical protocol repository's `schemas/`,
and `EXPERIMENT_PROTOCOL_FIXTURE` pointing to its
`examples/experiment-protocol/ecbench-l2-smoke-subset/protocol.json`.
The workflow pins that repository to `6a69b2d02ecd53f1649bcf56e23e4ffe91d953ad`.
Missing fixtures fail the tests; they are not skipped.

`plan` validates and claims; `run` submits or collects a claimed plan; `publish`
opens PRs. `export-ecbench` copies native benchmark metadata into the canonical
export schema, and `fetch-verified` downloads a bounded, hash-verified native
manifest. These last two commands run in the trusted CodeBuild controller.

The tests use fake GitHub/AWS/MCP services and fixture observations. They do not
authorize spending or demonstrate live EKS qualification. See the deployment
guide and [validation record](../../docs/experiment-queue/validation.json).
