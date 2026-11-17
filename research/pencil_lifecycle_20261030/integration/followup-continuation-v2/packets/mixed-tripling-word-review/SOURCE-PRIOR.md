# Independent retained-source observations before word packet inspection

Retained Field::mul checks tables before p/n: line207 table lookup can index unreduced operands directly; n=1 no-table path at220 calls bm, whose result is UInt64 (a*b)%p at191-193. The reviewed word bridge must therefore bind n=1, tables=None, odd prime p>=5 rather than claim every Field backend. Producer confirmed this explicit scope before seal. The actual validate envelope rejects odd primes >=2^31 and primes below5.

For dynamic canonical a<p<2^31 and a source literal c<=10, c*a<10*2^31<2^35<2^64. This is separate from canonical dynamic product a*b<p²<2^62. It permits unreduced literal operands5/8/10 for p5/p7 without silently using c<p. Result modulo positivep is canonical. Add/sub dynamic inputs must remain canonical; the special literal multiplication bridge cannot establish canonical literal inputs to arbitrary addition or indexing.

Source literal inventory for triple: fp factors2,3,5; fpp outer2 and inner3,10; k8; aa2; bb2; cc2; q3factor3; r3factors4,3; s3factor2. Each is a multiplication operand. No literal8/10 appears directly as an additive operand. No pow loop occurs in this triple function: sq values must decode as multiplication of identical canonical inputs. Source retains counters, excluded by value-only scope.
