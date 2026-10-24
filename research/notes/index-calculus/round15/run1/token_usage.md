### Token usage: 3f26b2c1-a4c8-591a-a1d4-67483220b91c

| agent | type | model | tier | requests | prompt tok | cache hit | output tok | cost | output share of cost |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| agent-aa9ad6eac3441753f | subagent | `claude-opus-5-5` | core | 40 | 5,262,345 | 96.5% | 20,630 (≥) | $2.3393 | 17.6% |
| main | main | `claude-opus-5` | core | 15 | 1,418,268 | 95.1% | 4,221 | $1.4754 | 7.1% |
| main | main | `claude-opus-5-5` | core | 532 | 226,741,834 | 98.2% | 373,648 | $85.2410 | 8.8% |
| agent-a110831d390c7d691 | experiment-director | `claude-sonnet-5-5` | director | 48 | 3,647,061 | 75.7% | 38,198 (≥) | $3.1502 | 12.1% |
| agent-a1a3a9a634d7fa612 | subagent | `claude-haiku-5-5` | mechanical | 4 | 74,747 | 44.5% | 5,551 | $0.0083 | 33.5% |
| agent-a1ad3f9b0107077af | subagent | `claude-haiku-5-5` | mechanical | 4 | 64,830 | 45.5% | 1,176 (≥) | $0.0053 | 11.1% |
| agent-a340cd941410160f2 | subagent | `claude-haiku-5-5` | mechanical | 4 | 75,192 | 44.5% | 5,690 | $0.0084 | 33.9% |
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
| **total** | | | | 702 | 238,347,669 | 97.6% | 477,143 | **$92.2794** | 9.1% |

- Without prompt caching the same tokens would cost $950.9233 (caching saved 90.3%).
- Director and mechanical tiers produced 16.5% of output tokens for 3.5% of cost.
- outcome **None**, $92.2794 and 477,143 output tokens per card.
- Note: agent a110831d390c7d691: final usage does not match its last response; output left as a lower bound
- Note: 15 subagent response(s) recorded without a final usage: their output_tokens are lower bounds
