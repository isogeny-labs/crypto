# SAT search terminal independent source review

Reviewed immutable manifest `94b9591b0abde424dc9c49dce38838221bc314537d579303b8552f4ae2f33aa7`, patch `39b06e1194dd5279504132f7597f03e99991db8378f4225847ea48cdd2da0da7`; all seven manifest artifact hashes verified. Source-only review, no main/target edits, build, native execution or cloud actions.

Decision: accept for isolated composition/compilation and the stated bounded controls. No concrete source blocker found for the limited producer-terminal/report contract. This does not provide universal durable failure finalization or central/export acceptance.

## Exact accounting and authority

Finalization runs once after every ordinary continue_search return, including setup/run refusal. Native outcome is retained immediately after solve before later deadline/observer checks. Existing original owner, cumulative Budget, solver, observer and deadline remain in place. No failed budget is cleared and no new allowance is minted.

The healthy path reserves work=4096*56+65536, adapter bytes2048, and trace=4096*4+4096 before copying receipt positions and constructing its fixed borrowed record. CAP applies to the entire encoded capture line, including envelope/newline, through the existing bounded sink API. Reused TypedEventBuffer capacity originates in prior owner setup; this patch does not allocate a new payload buffer. Wire totals are snapshotted after this prepayment. Encoding/commit callbacks impose only the pre-admitted cap, and successful publication performs no subsequent counter increment; thus successful wire totals and retained final cumulative counters agree at source scope. These are reservation units, not measured operations or actual output-byte totals. Source resembles the existing initial checkpoint tariff; actual cap/serialization tests still required.

Expected covered position comes from actual result acknowledgment, then actual search acknowledgment, then initial checkpoint. It is compared to current sink before publication. Record binds originals/pattern, initial checkpoint and immediately preceding endpoint; signed-relation and native-outcome claims remain distinct, with scalar/global-negative false.

## Refusal and custody semantics

Actual post-sink position is read before mapping sink error. If it differs, terminal_ack is retained even when sink reports failure or the subsequent original-deadline check fails. Validity separately requires exactly one added event, unchanged capture binding and increasing byte end. A changed/invalid position can therefore make terminal_acknowledged true while terminal_accepted is false; consumers must preserve this distinction rather than treat acknowledgment presence as validity.

Stopped owners fail the initial budget check before any new publication and retain only the local fixed-size report. Work paid before later trace refusal remains in report totals. First-stop provenance is preserved through budget.fail; terminal refusal does not overwrite the original failure. A successful checked model may remain checked_signed_relation=true while terminal publication fails: this is local mathematical progress, not committed/exportable success. Existing verdict() alone cannot authorize future export; whole terminal/report/capture/replay joins are mandatory.

The report is retained on ordinary function return, not panic/process kill. Missing durable terminal for Unknown/resource/capture refusal is explicitly acknowledged by DESIGN, not silently reclassified. A durable failure-finalization mechanism would need an originally admitted design; this patch does not create one.

## Required runtime/integration gates

The extended genuine both-twist closed-gate test only checks Unknown and unavailable terminal. Before activation require real healthy model/Sat terminal, uncertified Unsat terminal, original work/bytes/trace/addition/deadline refusal, actual sink failure before and after commit, expiry after acknowledgment, oversized terminal/cap refusal, duplicate finalization, model failure after native Sat, and telemetry failure after durable terminal. Preserve actual original owner and limits in all controls.

Central replay must independently bind terminal limits to the original launch/checkpoint, cumulative totals to prior paid totals, exact covered/result/search endpoint, final capture head and manifest. It must reject a terminal whose payload looks positive but whose postchecks/outer report failed. Existing central segment proposal has no terminal parser yet. No causal training, global Unsat, scalar result, new dispatch or public feature is authorized by this producer report.
