### Token usage: 3f26b2c1-a4c8-591a-a1d4-67483220b91c

| agent | type | model | tier | requests | prompt tok | cache hit | output tok | cost | output share of cost |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| agent-a948015e9f5521fda | subagent | `claude-opus-5-5` | core | 81 | 13,917,672 | 86.8% | 73,661 | $13.0681 | 11.3% |
| agent-aa9ad6eac3441753f | subagent | `claude-opus-5-5` | core | 40 | 5,262,345 | 96.5% | 20,630 (≥) | $2.3393 | 17.6% |
| agent-ab60f1398e4e4429c | subagent | `claude-opus-5-5` | core | 57 | 10,772,253 | 88.9% | 64,024 | $9.1599 | 14.0% |
| agent-ace31b6900bd2f3b7 | Explore | `claude-opus-5-5` | core | 35 | 4,190,363 | 96.4% | 19,382 | $1.9408 | 20.0% |
| main | main | `claude-opus-5` | core | 15 | 1,418,268 | 95.1% | 4,221 | $1.4754 | 7.1% |
| main | main | `claude-opus-5-5` | core | 681 | 315,647,207 | 98.3% | 457,825 | $114.9865 | 8.0% |
| agent-a110831d390c7d691 | experiment-director | `claude-sonnet-5-5` | director | 59 | 5,101,739 | 79.8% | 46,824 (≥) | $3.8524 | 12.2% |
| agent-a1a3a9a634d7fa612 | subagent | `claude-haiku-5-5` | mechanical | 4 | 74,747 | 44.5% | 5,551 | $0.0083 | 33.5% |
| agent-a1ad3f9b0107077af | subagent | `claude-haiku-5-5` | mechanical | 4 | 64,830 | 45.5% | 1,176 (≥) | $0.0053 | 11.1% |
| agent-a340cd941410160f2 | subagent | `claude-haiku-5-5` | mechanical | 4 | 75,192 | 44.5% | 5,690 | $0.0084 | 33.9% |
| agent-a3df71b77d94362c1 | subagent | `claude-haiku-5-5` | mechanical | 4 | 94,409 | 55.7% | 22,911 | $0.0172 | 66.5% |
| agent-a4d8f4b80913ecd31 | executor-mechanical | `claude-haiku-5-5` | mechanical | 20 | 454,384 | 93.5% | 6,840 (≥) | $0.0114 | 30.1% |
| agent-a65279af97ae99e32 | subagent | `claude-haiku-5-5` | mechanical | 5 | 93,877 | 55.0% | 5,868 | $0.0087 | 33.6% |
| agent-a658e0b6ba0d7ee55 | subagent | `claude-haiku-5-5` | mechanical | 5 | 94,763 | 64.6% | 6,061 | $0.0078 | 38.7% |
| agent-a6ba92cf37a49479c | subagent | `claude-haiku-5-5` | mechanical | 5 | 92,561 | 89.3% | 1,280 (≥) | $0.0027 | 23.7% |
| agent-a7e91674e5bd379cf | subagent | `claude-haiku-5-5` | mechanical | 2 | 30,194 | 85.4% | 524 (≥) | $0.0011 | 24.5% |
| agent-a83550fbc166e5941 | subagent | `claude-haiku-5-5` | mechanical | 2 | 29,703 | 74.1% | 1,126 | $0.0017 | 32.3% |
| agent-a9d8324797bf019fc | subagent | `claude-haiku-5-5` | mechanical | 2 | 32,538 | 44.4% | 1,048 | $0.0029 | 17.9% |
| agent-ab17784e12dde2cdb | subagent | `claude-haiku-5-5` | mechanical | 2 | 32,377 | 44.3% | 879 | $0.0028 | 15.5% |
| agent-ade7f14e057c3df37 | executor-mechanical | `claude-haiku-5-5` | mechanical | 4 | 64,270 | 72.8% | 1,838 | $0.0036 | 25.7% |
| agent-ae1ed374d00b89444 | executor-mechanical | `claude-haiku-5-5` | mechanical | 2 | 32,274 | 44.4% | 1,169 | $0.0030 | 19.7% |
| agent-af0616eb4e954d654 | subagent | `claude-haiku-5-5` | mechanical | 5 | 93,558 | 68.2% | 1,391 (≥) | $0.0050 | 13.8% |
| agent-af72733ff99395d27 | subagent | `claude-haiku-5-5` | mechanical | 1 | 12,893 | 69.7% | 5 (≥) | $0.0006 | 0.4% |
| **total** | | | | 1039 | 357,682,417 | 97.1% | 749,924 | **$146.9131** | 9.1% |

- Without prompt caching the same tokens would cost $1,429.9073 (caching saved 89.7%).
- Director and mechanical tiers produced 14.7% of output tokens for 2.7% of cost.
- outcome **None**, $146.9131 and 749,924 output tokens per card.
- Note: agent a110831d390c7d691: final usage does not match its last response; output left as a lower bound
- Note: 15 subagent response(s) recorded without a final usage: their output_tokens are lower bounds
