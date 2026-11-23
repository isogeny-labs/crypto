# Admission review correction before delivery

Root's independent source review of8e87ba53e62679c861a40816c0485dab54cf482b
found that generic JSON trees were fully allocated by the old common parser
before decoded-memory checks. The32MiB v2 check also followed the legacy64MiB
loader. Earlier validation is retained and does not establish pre-allocation
admission. No producer fixture is regenerated for this consumer correction.

Replace new-v2 dispatch with a streaming, no-tree, SHA256-bound schema classifier.
The classifier reads at most the legacy64MiB cap to preserve v1 classification,
then v2 rejects actual metadata size above32MiB before raw/Value loading. A second
non-materializing DeserializeSeed pass counts the conservative JSON tree budget
before invoking the duplicate-key Value decoder. Charge before reading each
child; stop traversal immediately when the remaining model budget cannot admit
it. In addition to64bytes per Value, charge128bytes per array slot (conservative
capacity growth),512bytes per object member plus twice key/string byte length.
Raw bytes, two admitted tree copies, native sparse scratch, and rank temporaries
share the512MiB modeled-storage ceiling. Stream/decoder scratch and allocator
metadata remain explicitly excluded; this is not an RSS proof. Physical process
memory is bounded separately by the software supervisor.

Rank allocation accounting includes up to64bytes per dense coefficient for
numeric rows and newly produced canonical decimal strings, plus row metadata.
The correction also reserves128bytes per row and64bytes per column for RHS,
outer-row, pivot, determination and value vectors; an empty-row matrix still
requires its column storage before allocation.
Native sparse accounting additionally reserves completed rows, source maps,
active row copies, pivot snapshots and source-combination accumulators before
cloning them. Model rejection remains consumer_resource_limit, never producer
censoring or a mathematical result. Existing logical/curve/dense-cell caps stay
unchanged. Tests must show a tiny model budget stops a nested sequence before
traversing the remainder, and new-v2 loading rejects over32MiB before decoding.

The saved-fixture summary also counted a stray unsealed legacy transcript file
as route coverage. The corrected summary will require the manifest-admitted
role and a verified outcome; historical report5 includes that unsealed file,
while the primary manifest evidence has4 verified legacy Koblitz cells.

Classifier string scratch remains bounded by the existing64MiB legacy raw cap
so an ordinary legacy large string retains its former admission. The classified
new-v2 loader uses a1MiB raw-string ceiling before serde parses it; both stages
bound raw nesting to128. Schema keys are streamed without retaining key strings,
duplicate/escaped duplicate schema keys and nonstring schemas fail before routing,
and strict EOF plus a complete streaming SHA256 binds classification to the later
read. The preflight visitor returns unit and keeps no array/map/value containers.
Existing public/base/config/positive-certificate parsing outside this new module
retains its separate legacy limits and is not claimed as covered by its512MiB
modeled-storage ceiling.

The resumed implementation checked the official Serde
[DeserializeSeed documentation](https://docs.rs/serde/latest/serde/de/trait.DeserializeSeed.html)
and serde_json [reader and EOF documentation](https://docs.rs/serde_json/latest/serde_json/struct.Deserializer.html)
on2026-10-29. The stateful unit-returning visitor admits structure without
retaining containers; reader classification uses explicit buffering and calls
end() to reject trailing data. These are parser mechanics, not cryptographic
novelty claims or an RSS measurement.
