# Catalog integration CI follow-up

The current-head native parity/audit workflow
[39668445853](https://github.com/aburan28/crypto/actions/runs/39668445853)
passed the full finite replay, independent certificate checks, corruption controls,
native backend tests, exact metadata replay, and receipt upload.
The validation gate and perf-history workflow also passed on
`4711f1c73887f01ef8f348d98d57e0c09b221afd`.

The cover workflow
[39668445455](https://github.com/aburan28/crypto/actions/runs/39668445455)
found integration failures caused by this PR's 1,921 added registry models:

- Job 119459599073: the catalog-wide assessment's source binding was stale.
- Job 119459599406: the standalone constructor test expected 2,572 models;
  the current registry contains 4,493. Its other 12 CLI tests passed, and
  the native cover checker's all-model deterministic replay passed.

The local failing reproductions are retained in `assessment-before.log` and
`constructor-before.log`, with separate exit files. `inputs-before.sha256`
binds the original source and metadata. This follow-up refreshes the native
cover report, content-addressed graph, and assessment for the expanded registry.
Every per-model constructor and semantic assertion remains in force.

The earlier dated reports, frozen mathematical results, source archive, and
historical receipts are not rewritten. The current coverage report is versioned
under `../../reports/catalog-20261022/`. This is a catalog consistency correction;
no speed, general hardness, transfer algorithm, or solver claim follows.

Shared Conductor remains unavailable: the check attempted before this fix
exited 127 (`conductor: command not found`), and `.conductor/WORKFLOW.md` is
not tracked in this repository. No reservation is claimed and no replacement
coordinator was started. The original formal and coordination obligations remain
open in the problem record.

## Verified results

| Check | Result | Record |
| --- | --- | --- |
| Native cover generation and byte-exact replay | 4,493 verified; zero invalid or unsupported | `cover-generate.log`, `cover-check.log`, exit 0 |
| Full release library suite | 4,474 passed; 119 existing ignores | `cargo-tests.log.xz`, exit 0 |
| Cover checker / standards / standalone binaries | 11 + 2 + 13 tests passed | same combined Cargo transcript |
| Native standards byte replay | 258 imported, seven existing unresolved records | `standards-check.log`, exit 0 |
| Deterministic assessment | byte-exact and current | `assessment-check.log` |
| Draft 2020-12 schema | valid, with AJV CLI 5.0.0 and ajv-formats 3.0.1 | `schema-check.log` |
| Independent assessment | zero problems; 4,493 maps, 4,491 covers, 1,004 EC1 records, 538 field contexts | `assessment-verify-final.json`, exit 0 |
| Missing extension-map membership | rejected with seven coverage, partition, and digest findings | `membership-control.json`; control exits 0 only for the expected rejection |
| Existing-record preservation | all 2,572 prior model findings/maps, 2,570 cover objects and 977 EC1 representations retained | `catalog-preservation.json` |
| Claim hygiene | no support status changed; seven population-scope wording updates in the six pinned claim sections | `semantic-refresh.json` |
| Report | three pages and both diagrams visually inspected; exact coverage marker verified | `pdf-build-final.log`, `../../reports/catalog-20261022/` |

The registry itself is byte-identical to `4711f1c73887`. The generated native
catalog inputs are pinned at `a11984a573441d5bf0d60ab1d26cfc34e71badf5`, so the
assessment verifier's Git-blob checks remain active. Every per-model constructor,
membership, source, formula, and field/context assertion is retained.

`assessment-verify-attempt1.json` retains the first full assessment replay's
rejections of two remaining old count guards, a stale wrapper-source digest in
the infrastructure guide, and a stale graph label. The corrected guards use
4,328 odd constructions and 2,041 prime-field replays; the wrapper's documented
digest is recomputed from its exact three producer inputs. No semantic result
was promoted to obtain the pass.

The first PDF build dropped unescaped underscores from its Subject marker;
its source, PDF and log are preserved. The next graph draft and its PDF are also
retained; visual review found old population labels. The final PDF and diagrams
use the verified counts and retain the original transfer/solver limitations.

The membership control's first fixture used symlinked inputs and hit the
verifier's realpath-containment checks before its intended assertion. A hard-link
fixture then hit a cross-filesystem `EXDEV` error. Both failed setup scripts,
stderr and exits are retained. The final control copies the pinned inputs into a
fresh directory, preserving all containment checks, and alters only one exact
map membership. Its seven findings explicitly identify the removed membership;
the positive assessment has zero findings.

`commands.md`, `environment.log`, and `inputs-after.sha256` record reproduction,
tool versions, and final source/output bindings. Compressed logs are read with
`xz -dc FILE.log.xz`. Current-head CI still must complete on the resulting commit;
these local passes do not discharge any open formal or coordination obligation.
