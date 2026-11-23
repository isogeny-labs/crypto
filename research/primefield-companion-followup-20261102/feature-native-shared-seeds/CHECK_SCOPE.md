# Shared-seed native integration contract

One rustc build (120 s) and one filtered seed_tests invocation (30 s), one attempt.
Three methods fixed before execution: five 16-point windows on F509 starting
O,P,2P,-P,3P with direct-phase seed equality, checker-only Euler signs and saved
public-point fixtures; repeat a prepared O-start length-five window on each of
three original curves and verify calibration/raw records, empty and invalid input;
F7/r13 and F5/r7 length-five windows exercise omitted singleton and omitted partial
branches. Dependency and prior integration methods are compiled but filtered out.
No timing comparison or full4563-case cohort. Logical method counters, not complete
field costs. The direct phase reference is test-only and excluded from method counts.
