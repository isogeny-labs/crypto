# Corrected receipt patch: v2.1

This is the applicable successor to preserved v2. Root source review caught a broad replacement in one free test expectation (`self.witness` after projecting receipt.commit). V2.1 fixes that test expectation only; production code/API remain as specified in dedicated-witness-capture-v2-receipt-contract.md and v2 handoff. Do not apply v2 e413b195.

Base:9bfe5bcae18fb3007730bf5bae8cea098138647d.
Full patch:dedicated-witness-capture-v2-1.patch SHA256183928140b29133e5d866b54366e2fe186d4068a41b3e5ac4fe5e198a4552992.
V1-to-v2.1 delta:dedicated-witness-capture-v2-1-delta.patch SHA25676779689e9230b74dc95362341bba2c13a903e749f946d6784bded48d7a7b8bc.

Reviewed complete corrected delta. rustfmt --check and git apply --check passed. No Cargo or tests run, no repository changes. Root retains serialized application/validation ownership. V1 and v2 preserved for custody. No new receipt on IndeterminateIo, no payload allocation, fixed requested helper receipt layout added; caller persistent receipt still needs admission.
