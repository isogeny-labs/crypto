# Square-root primitive scope correction

This additive correction applies to the frozen four-corner Delta report and its prospective-plan wording about enumeration. The new Mumford lift construction forms at most four candidates from square roots and remainder equations; it does not enumerate all possible Mumford coefficient combinations or group/scalar values.

The reused Field.sqrt primitive does perform a finite-field nonresidue search: native/src/three_leads.rs contains `(2..q).find(|&z| self.pow(z, (q-1)/2) != 1)` in its prime-field square-root path. Therefore the report sentence “No field, group or scalar enumeration occurs” is too broad. Square-root work, including that search, remains part of the execution and must be charged in an eventual complete cost model. Native counters are preserved, but they are not an independently complete accounting of all exponentiation/search work.

No source, run, counter, or frozen report is changed. No rerun is required for this interpretation correction. Candidate-count boundedness and finite checked outcomes remain as recorded.
