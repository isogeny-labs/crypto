# Director assessment — CARD-oracle-leads-20261019-1

Assessment scope is execution, binding, receipt, relay and recorded write
scope only. It does not interpret the experiment or admit any performance,
regularity or cryptographic claim. Recovery completed from saved records;
no research command was rerun.

## Conclusion

The recorded run executed the three frozen commands as specified: each has
one attempt, exit0, no card-level timeout, matching arguments and working
directory, and every declared output present with its recorded digest.
The executor relay is byte-exact to the output produced by the unchanged
canonical `brief()` using the output-map insertion order constructed by
the unchanged canonical `run_card()`.

**The canonical `check-relay` CLI did not pass.** Its preserved exit1 is a
receipt serialization/order defect described below. It must not be reported
as a successful canonical CLI check or hidden by rewriting the relay,
receipt, card or tool. The narrower read-only verification supports
`executed_as_specified` and `relay_verbatim_to_actual_runner_output: true`;
the unchanged canonical assessment API itself still reports
`executor_deviated` because it inherits that order-sensitive CLI result.

## Verified records

- Frozen source commit: `aaf9e246a3af4be371d7089dc86fc32c61c2f5ac`.
- Card SHA256: `0b861eb6573f5c1e98a0ea792d6a5fcfa420ed15bec7d43bab315fe664768f5a`.
- Receipt SHA256: `bae4f32726b6cc2cf5f26b8383546421c8b1fd02411735a622540f3c54d79ab0`.
- All24 source, input, protocol, plan, canonical-tool and executable bindings
  match the unchanged card.
- All3 commands ran once and succeeded; all6 recorded stream digests and
  tails, and all5 declared output-digest occurrences, match current bytes.
- All2,493 sealed experiment members match their hashes. No extra or
  missing file was found in that sealed experiment directory.
- No recorded new Git dirty path lies outside the declared study scope.
  All declared streams, outputs, receipt and sealed members separately
  resolve inside that scope.
- Relay SHA256 and reconstructed actual runner-output SHA256 are both
  `9714d40957e5192796bfaa27b4a03761eaf2574fbade8b1b2ce6bdf5f149fcc1`.

The initial executor dispatch refusal requested an exact absolute repository
root. The parent supplied `/workspace/crypto-pr2134-head819-review` before
any native attempt. The frozen card, command arguments, limits and no-retry
rule were unchanged.

## Why canonical check-relay fails

`tools/task_card.py` is unchanged (SHA256
`f7284c366ecf14d2a7148cf704fbe4eda61060de666e34ac9d9257815d848e8b`).
Its `run_card()` constructs each output dictionary in the card's declared
order, writes the receipt with `json.dumps(..., sort_keys=True)`, and returns
the original in-memory receipt. The run then prints `brief()` from that
in-memory receipt. The first two commands declare `summary.json` before
`leads-manifest.json`, so that is the actual printed order.

`check_relay()` reloads the sorted serialized receipt, and `brief()` consumes
the resulting dictionary order. It therefore expects `leads-manifest.json`
before `summary.json`. Its line matcher requires ordered occurrences and
reports missing lines when comparing the correctly relayed original output.
All values and lines agree; only the two output-line orders differ.

The original failed command, stdout and stderr remain in
`check-relay-command.json`, `check-relay.stdout` and `check-relay.stderr`.
`canonical-reloaded-brief.txt`, `canonical-runtime-order-brief.txt` and
`relay-order.diff` retain both forms and their difference. No canonical
source, scientific output or immutable receipt was changed to resolve this.

## Reproduction and limits

Read-only helper:

```sh
python3 research/factor_base_oracle_leads_20261019/cards/verify-assessment-recovery.py
```

Its saved output is `assessment-recovery.json` (exit0). It imports and calls
the unchanged canonical `load_card`, `brief`, `check_relay`,
`assessment_facts`, `_tail` and scope/path helpers, then reorders only an
in-memory copy of output dictionaries according to the unchanged card.
The original broader proof is preserved as `assessment-verification.json`.
No experiment execution, native arithmetic, source edit or retry occurs.

Git dirty-path snapshots do not enumerate ignored files. The separate
declared-file and manifest checks cover the known ignored outputs, but this
is not process tracing or proof against every possible undeclared write.
No independent-host execution or formal certificate is established.
