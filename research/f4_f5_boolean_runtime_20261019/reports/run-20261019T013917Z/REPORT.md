# Launch capacity refusal

Run 20261019-013917-boolean-native-baseline-9a4ea0: the on-demand c7i.2xlarge request was rejected by the shared 32-vCPU Standard On-Demand quota before any instance booted. No compilation, timing or solver result exists for this attempt.

Raw launch/status/command receipts: preparation-failures/quota-baseline-*. Dedicated retries use the same c7i.2xlarge type in the separate Spot quota. The frozen inputs, one-worker/core reservation, output verification and admission gates remain unchanged. No other job was stopped and no account setting changed.
