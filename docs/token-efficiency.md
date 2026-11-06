# Token efficiency of the agent workflows

`tools/token_usage.py` measures what the repository's agent workflows spend
in tokens and dollars, from the usage the runtimes already record. It reads
token counts only, never message content, so its records can be committed
and uploaded as CI artifacts (CLAUDE.md: no chat transcripts as metadata).

| workflow | what is read | command |
| --- | --- | --- |
| Model-tier relay (AGENTS.md "Model tiers"): main session, `experiment-director`, `executor-mechanical` | the session transcript `~/.claude/projects/<project>/<session>.jsonl` and `<session>/subagents/agent-*.jsonl` | `python3 tools/token_usage.py session [<session>.jsonl] --units <cards> --unit-name card` |
| `autofix-queue.yml` (Haiku, then Sonnet) | `steps.<attempt>.outputs.execution_file` of `anthropics/claude-code-action` | run by the workflow; records land in the job summary and the `token-usage-pr<N>-…` artifact |
| Any set of records | a JSONL ledger written with `--ledger` | `python3 tools/token_usage.py summary <ledger.jsonl>` |

`session` with no path reads the newest transcript under `~/.claude/projects`.
Every command prints a Markdown table; `--json` writes the record
(`crypto.autoresearch.token-usage.v1`), `--ledger` appends it to a JSONL
ledger, `--svg` writes a cost-by-component chart.

## Metrics

Per agent and model, and in total:

| metric | definition |
| --- | --- |
| `requests` | API responses, deduplicated by message id |
| `prompt_tokens` | uncached input + cache writes (5 min and 1 h) + cache reads, summed over requests |
| `cache_hit_ratio` | cache reads / prompt tokens |
| `mean_prompt_tokens`, `max_prompt_tokens` | context carried per request; growth here is what makes long sessions expensive |
| `output_tokens`, `thinking_tokens` | thinking is a subset of output, billed as output |
| `cost_usd`, `cost_usd_by_component` | list price per request from `tools/token_prices.json` (Haiku 5.5 long-context tier and fast mode applied per request) |
| `output_cost_share` | output cost / total cost |
| `uncached_cost_usd`, `cache_saving_ratio` | the same tokens with every prompt token at base input, and 1 − cost / that |
| `delegated_output_share`, `delegated_cost_share` | share of output tokens and of cost produced at the director and mechanical tiers: how much of the work the relay pushed below the reasoning tiers |
| `outcome.*_per_unit` | with `--units N`: cost and tokens per delivered unit (a card, an accepted fix) |

`summary` adds `cost_usd_per_accepted`: the spend of every recorded run,
rejected and stale attempts included, divided by the accepted ones. That is
the number to compare across models or prompt changes. A cheaper attempt
that fails more often can cost more per accepted fix.

## Accounting traps the tool handles

Each of these was found in real transcripts, and each changes the answer:

1. **One response, several lines.** Claude Code writes one transcript line
   per content block (thinking, text, each tool call), and every line
   carries the whole response's usage. Summing lines overstated this
   session's output tokens 2.17× and prompt tokens 1.94× (65 lines, 34
   responses). The tool counts each message id once.
2. **Subagent output is a streaming snapshot.** Subagent transcripts record
   usage when a response starts: one recorded 3 output tokens where the API
   billed 212. The tool replaces the last response's output with the
   Agent tool result's `usage` (matched on identical prompt size). Other
   responses with no `stop_reason` keep the snapshot. They are counted
   as lower bounds and flagged: `(≥)` in the table and
   `output_lower_bound_requests` in the record.
3. **`totalTokens` is not spend.** The Agent tool result's `totalTokens` is
   the final request's context size (input + cache + output of one
   response), not what the subagent consumed. It is never used.
4. **Cache-read rates differ by model.** Opus 5.5 reads at 0.05× input, Fable
   5.1 at 0.025×, Sonnet 5.5 and Haiku 5.5 at 0.1×, so one multiplier misprices
   long sessions. Prices are per model in `tools/token_prices.json`, with
   their source and date.
5. **Two CI attempts, one execution file.** Both claude-code-action steps in
   `autofix-queue.yml` write `$RUNNER_TEMP/claude-execution-output.json`, so
   the workflow records the Haiku attempt before Sonnet can overwrite it.
   When the action's per-response usage and the result's `modelUsage`
   disagree, `modelUsage` (the SDK's own total) wins and a note says so;
   `reported_vs_priced` compares our price with the runtime's
   `total_cost_usd`.

## Limits

- Prices are list prices as of the date in `tools/token_prices.json`, not
  invoices. Batch discounts and negotiated rates are not applied.
- A subagent's intermediate output tokens are lower bounds until Claude Code
  records final usage in subagent transcripts. Input-side tokens are exact,
  since they are known when a request starts.
- An attempt that wrote no execution file (it died before the action
  finished) has no record, so its spend is missing from
  `cost_usd_per_accepted`. The job summary says "No execution file was
  written" for it.
- In CI, "accepted" means the attempt passed `autofix_run.py verify`, not
  that the pushed fix later turned CI green.
- On the claude-code-action path, `modelUsage` has no cache TTL split, so
  cache writes there are priced at the 5-minute rate when per-response usage
  is missing.
- Aggregated usage cannot determine whether individual requests crossed a
  long-context pricing threshold. Such costs remain unknown when the total
  prompt exceeds that threshold. Sanitized files with no usage are also
  unknown, not free. If any ledger record is unpriced, the summary reports
  only a known subtotal and leaves total cost and cost per accepted outcome
  null, including when the unpriced record was a rejected attempt.
- Model prices match exact IDs, optionally followed by an eight-digit dated
  snapshot suffix and/or `[1m]`. Other version or preview suffixes remain
  unpriced even when they share a prefix with a known model.

## Tests

`python3 tools/test_token_usage.py` covers deduplication, the subagent patch
and lower-bound flag, per-component pricing, the Haiku long-context
threshold, fast mode, the action file (both paths) and the ledger summary.
