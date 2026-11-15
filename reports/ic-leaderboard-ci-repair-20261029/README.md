# IC leaderboard CI repair

The configured main-push workflow failed at input coverage: the checker also
required a pull-request event that the owner intentionally removed, and two
new study input families were absent from the push filter. Check every
configured push/PR event, distinguish absent events from unfiltered events,
and refuse empty/unsupported path configurations. Preserve complete input
coverage. Add post-GB and twist-study paths to the existing main-push event;
no PR trigger is added.

All 16 unit controls pass. Actual builder tracing covers all 860 opened input
files with the configured push filter. The regenerated JSON, Markdown and
HTML pass byte-currentness. Comparison preserves every scientific section,
all 20,592 existing roster objects, and every progress-point value. It adds
47 current-registry roster records, updates only registry/progress source
hashes, and changes progress-point presentation order. No frozen input,
validation threshold, result or admission policy changes.

Local validation first exposed missing sparse-checkout inputs. The exact
tracked prices, validation records and validator raw-data bindings were
materialized before generating the accepted output. An early comparison
correctly refused a validation demotion caused by missing raw files; those
inputs were restored and the existing post-GB validation passes with zero
errors. No scientific demotion or promotion from an incomplete checkout is
committed. The compact accepted data comparison and raw passing logs are
retained here.

Sources checked: current builder/read recorder, validation gate, actual
main-workflow failure job 123228689314, repository owner merge policy, and
GitHub's official workflow-syntax documentation:
https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax
No experiment or timing comparison was run. Current-head hosted merge checks
and fresh review remain required.

The workflow's final lab-browser check independently exposed an index still
at 20,592 catalog curves. Regeneration uses the existing Git-aware builder
and adds exactly the same 47 catalog curves. All 20,592 existing curve
objects are byte-semantic identical, and every non-curve scientific section
is unchanged. Only counts and input-source bindings change beside those
additions. Browser currentness and the existing browser/leaderboard parser
suites pass. No browser UI or generator algorithm changes.

## Latest accepted main: 20,787 rows

After normal integration of concurrent main synchronization, both views add
195 curves relative to original main d0c081c8, with zero removals. Existing
numeric model, order, generator and result fields remain identical. Accepted
main adds two EC1 representations and aliases for one existing curve and
refreshes 121 source-reference lists. That curve also receives its canonical
generator-construction label and expanded record-UID list. Progress-point
values are unchanged; the source's updated-date metadata advances from
October 27 to October 28. The exact field-level comparison records these
inherited metadata changes. All non-curve scientific sections are unchanged.
Currentness of all four outputs and complete 860-file coverage pass again.
Missing Git objects were fetched by exact named input; the generator's strict
missing-input refusal remains active.
