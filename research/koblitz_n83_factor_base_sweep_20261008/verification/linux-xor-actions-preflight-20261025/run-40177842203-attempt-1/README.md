# Exact-access diagnostic, hosted run 40177842203 attempt 1

The provider job at protocol head `9b6d5f4bbc4bb9c80cf22a1c58bbfb288804f723`
passed the Linux solver smoke and failed its first `GetObject` probe before
building the worker. The retained caller identity, CLI stderr, exit status,
and provider job metadata establish the precise failure: the GitHub Actions
AWS user lacked an identity-based `s3:GetObject` grant for
`inputs/hash_5_1.xcnf`. This differs from the earlier run, whose wrapper
discarded the AWS error text.

These files were extracted with `gh run download` while attempt 1 was still
available. GitHub removed that attempt's artifact after the authorized rerun;
its exact provider ZIP now returns 404. These retained individual files and
provider metadata are therefore diagnostic evidence, **not** a replay of an
exact provider ZIP. No relation search occurred in attempt 1.
