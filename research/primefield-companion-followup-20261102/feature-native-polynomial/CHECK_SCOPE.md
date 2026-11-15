# Native polynomial software contract

One standalone rustc build capped120seconds; one native test invocation capped30seconds.
Four methods: three original source resultants and partial polynomial values;
schoolbook checks across cutoff12 and uneven lengths; Newton inverse identities
and independent classical long division; multipoint order/repeated points/Horner
agreement, common-root zero and domain errors. Explicit inputs fixed in source.
Native production retains Karatsuba/Newton/tree algorithms; schoolbook/long
references are independent unit-test controls. No Python execution path.

No sign-source recovery, timing comparison, cloud job or full cohort. Caller
certifies prime p; modulus<=u64::MAX/2. Test elapsed text is diagnostic. Source,
fixtures and original source/input hashes fixed before execution.
