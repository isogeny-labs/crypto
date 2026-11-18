# Native decoding of verified worker inputs

The staged decoder accepts the private verified-worker token and charges a fixed modeled checking allowance before inspecting its bounded input shapes or allocating retained data. It reads only canonical decimal u64 words of at most 20 digits, at most four original signed factor-base rows, and at most 61 field exponents. The signed domain is expanded and deduplicated in an eight-entry stack array and must reduce to the original two-code/four-point profile. Actual twist coefficients are retained.

The decoder constructs the existing native BinaryCurve using F2mElement::from_bit_positions, whose current Words implementation is inline at these field sizes. It does not repeat irreducibility, order or subgroup mathematics already checked by the worker. Historical independent original_math replay remains unchanged. The allocation allowance includes retained MathInput/curve layouts, curve construction temporaries, bounded exponent and bit-position arrays, point arrays, and BigUint layouts/backing allowances. It models requested storage, not allocator overhead or RSS; Box/BigUint remain existing native allocation APIs under the process supervisor.

New controls compare native decoded inputs against independent original replay at n=23,31,47,53,61 on both actual twists, and exercise exact/one-less work and memory admission and an expired setup deadline. Original budget binding in the final producer entry, root acknowledgement transfer, central dispatch and CLI activation remain separate integration requirements. The decoder is still compiled only under the staged adapter's test registration.

Validation logs and source hashes are stored separately. A pre-result review identified a likely fixture issue in the unbound-token negative control: it calls the telemetry-writing helper twice in one immutable directory. Preserve the first terminal result and, if that collision is confirmed, use a fresh fixture for that control without changing the decoder or limits.

The first broad run completed with 499 passes, one failure and two ignored tests.
The failure confirmed the predicted immutable telemetry collision in the unbound
negative control. The requested size/twist comparison passed, and the resource
boundary assertions completed before that fixture error. The corrected control
uses a fresh original-input fixture and registry; decoder code and resource
limits are unchanged. The failed logs and source remain separately archived.
