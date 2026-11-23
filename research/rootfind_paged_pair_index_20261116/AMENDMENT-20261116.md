# Editorial protocol amendment after source checks

The protocol was frozen at `d80f3e97f1` before the standalone prototype and
checks. After those checks, `pcc lint-prose` found that its final paragraph's
word "verified" lacked a local standing marker. The sole change to
`PROTOCOL.md` adds `[pcc:unverified]` to that paragraph. It does not alter the
mechanism, parameters, inputs, tests, acceptance rule, timing gate or
interpretation. The original frozen text remains recoverable at the cited
commit; this amendment records why the later text differs. [pcc:unverified]
