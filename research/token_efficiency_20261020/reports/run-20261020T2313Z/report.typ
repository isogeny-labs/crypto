#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#let r = json("record.json")
#let t = r.total
#let usd(x) = "$" + str(calc.round(x, digits: 4))
#let pct(x) = str(calc.round(100 * x, digits: 1)) + "%"

= Token usage run report: one Claude Code session

*Run:* session #raw(r.source.session_id), read 2026-10-20 at the session's state then (UTC start 23:13). *Commit:* `748420928` (base of the change). *Instrument:* `tools/token_usage.py session` at the PR head. *Prices:* `tools/token_prices.json`, list prices as of #r.prices.as_of. All values are *measured* token counts from the transcript. Costs are *derived* from those counts × list price per request.

*What ran:* the core-tier main session (Opus 5.5) that wrote this change, and one Haiku 5.5 `Explore` subagent dispatched once as a probe of how subagent usage is recorded. #t.requests API responses in all.

#table(columns: 8, align: (left, left, right, right, right, right, right, right),
  [*agent type*], [*model*], [*requests*], [*prompt tok*], [*cache hit*], [*output tok*], [*cost (USD)*], [*output cost share*],
  ..r.groups.map(g => (g.agent_type, g.model.replace("claude-", ""), str(g.requests), str(g.prompt_tokens),
     pct(g.cache_hit_ratio), str(g.output_tokens) + if g.output_lower_bound_requests > 0 { " (≥)" } else { "" },
     usd(g.cost_usd), pct(g.output_cost_share))).flatten(),
  [*total*], [], [*#t.requests*], [*#t.prompt_tokens*], [*#pct(t.cache_hit_ratio)*], [*#t.output_tokens*], [*#usd(t.cost_usd)*], [*#pct(t.output_cost_share)*],
)

#figure(image("cost.svg", width: 100%), caption: [Cost by component per agent group, session #r.source.session_id. One run, so no spread is recorded.])

== Derived values
- Uncached counterfactual (every prompt token at base input): #usd(t.uncached_cost_usd). Caching saving ratio #pct(t.cache_saving_ratio) = 1 − #usd(t.cost_usd) / #usd(t.uncached_cost_usd).
- Share of output produced below the reasoning tiers: #pct(t.delegated_output_share). Share of cost: #pct(t.delegated_cost_share).

== Observations
- Summing transcript lines instead of responses overstated output 2.17× and prompt 1.94× at 65 lines / 34 responses (measured mid-session with the same instrument).
- The subagent's recorded output for its last response was 3 tokens. The Agent tool result's billed usage said 212. The first response's 20 tokens stay a lower bound.
- The subagent's first request wrote 37,408 tokens to cache (its system prompt and tools), so its cache hit ratio is #pct(r.groups.at(1).cache_hit_ratio) over two requests.

== Limits
One session, one subagent dispatch: descriptive, no comparison. List prices, not invoices. Records: `record.json` beside this report.
