# cpu profile: First32 recorded Run51 coordinate systems; simplified unit reduction; not throughput

- status: **ok**
- command: `/tmp/ic-descent51-native/recorded-coordinate-profile /workspace/ic-runpod-runs/runpod-unit-deferred-20261117-51/returned/task/run/pair-unit-five.raw.json fast /tmp/ic-unit-coordinate-profile-20261123/profile-fast.coordinates.json`
- binary: `/tmp/ic-descent51-native/recorded-coordinate-profile` sha256 `263a70fc2c2d20bf…`
- commit: `92b9dae5db72` (0 dirty tracked files)
- host: AMD EPYC 9V74 80-Core Processor, 1 logical CPUs, kernel 6.18.57, PMU absent, perf_event_paranoid 2
- valgrind: valgrind-3.24.0
- isolation: none; wall 135.662 s (profiled run: not a timing measurement)
- started: 2026-11-23T07:02:57Z

## Hotspots (callgrind, self Ir)

Total: 29289230219 Ir (deterministic, simulated; not cycles).

| # | self % | Ir | function | object |
|---|---:|---:|---|---|
| 1 | 20.13 | 5897103346 | `sha2::sha256::compress256` | recorded-coordinate-profile |
| 2 | 12.4 | 3630516115 | `0x00000000000a0fe0` | libc.so.6 |
| 3 | 8.47 | 2479445088 | `free` | libc.so.6 |
| 4 | 5.51 | 1614111888 | `malloc` | libc.so.6 |
| 5 | 4.34 | 1271961913 | `0x00000000000a02e0` | libc.so.6 |
| 6 | 3.53 | 1032966526 | `0x000000000009ebd0` | libc.so.6 |
| 7 | 2.51 | 735827479 | `<serde_json::value::Value as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::SliceRead>>'2` | null |
| 8 | 1.98 | 579928293 | `<serde_json::de::Deserializer<serde_json::read::SliceRead>>::parse_any_number` | null |
| 9 | 1.93 | 564709935 | `0x000000000009eaf0` | libc.so.6 |
| 10 | 1.8 | 527304492 | `<serde_json::value::Value as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::SliceRead>>'2` | null |
| 11 | 1.72 | 502957206 | `<serde_json::number::Number as core::convert::From<u64>>::from` | recorded-coordinate-profile |
| 12 | 1.5 | 438583328 | `<serde_json::value::Value as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::SliceRead>>'2` | recorded-coordinate-profile |
| 13 | 1.4 | 409342479 | `<serde_json::de::ParserNumber>::visit::<<serde_json::value::Value as serde_core::de::Deserialize>::deserialize::ValueVisitor>` | recorded-coordinate-profile |
| 14 | 1.26 | 368570708 | `core::ptr::drop_glue::<[serde_json::value::Value]>'2` | recorded-coordinate-profile |
| 15 | 1.21 | 353319862 | `0x0000000000162880` | libc.so.6 |
| 16 | 1.13 | 330028856 | `<serde_json::read::SliceRead>::skip_to_escape` | recorded-coordinate-profile |
| 17 | 1.1 | 321118713 | `<serde_json::value::Value as serde_core::de::Deserialize>::deserialize::<&mut serde_json::de::Deserializer<serde_json::read::SliceRead>>'2` | null |
| 18 | 1.09 | 319142608 | `<serde_json::de::Deserializer<serde_json::read::SliceRead>>::scan_integer` | recorded-coordinate-profile |
| 19 | 1.07 | 314362108 | `__rustc::__rdl_alloc` | null |
| 20 | 1.07 | 312835442 | `<u64 as itoa::Unsigned>::fmt` | recorded-coordinate-profile |
| 21 | 1.06 | 310175848 | `<serde_json::de::Deserializer<serde_json::read::SliceRead>>::scan_integer` | null |
| 22 | 1.02 | 297694912 | `core::str::converts::from_utf8` | null |
| 23 | 0.94 | 274773443 | `<serde_json::read::SliceRead as serde_json::read::Read>::parse_str` | recorded-coordinate-profile |
| 24 | 0.92 | 268257729 | `<serde_json::de::Deserializer<serde_json::read::SliceRead>>::parse_any_number` | recorded-coordinate-profile |
| 25 | 0.91 | 265765030 | `0x000000000009ed80` | libc.so.6 |

## Files

- `annotate-inclusive.txt`
- `annotate.txt`
- `callgrind.out`
- `command.stderr`
- `command.stdout`
- `hotspots.json`
- `hotspots.svg`
- `manifest.json`

Raw profiler output (perf.data, *.ncu-rep, *.nsys-rep, callgrind.out) is large: push it with `crypto store push` (AGENTS.md section 13) rather than committing it.
