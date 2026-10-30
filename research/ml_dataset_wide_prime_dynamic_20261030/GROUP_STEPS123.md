# Group steps1–3 revision3 wire correction

Complete replacement patch against9bfe5bcae18fb3007730bf5bae8cea098138647d. Preservev1/v2.
Path dynamic-group-steps123-v3.patch.
SHA25677dd33ac5ae1cfb5eb26e8199a56c6d75dd36ac68bb9a4d2f57a54e251e04d6b.

All five u128 wire fields now serialize as canonical decimal strings using a borrowed40-byte stack formatter: GroupRow.cursor and GroupReport.cursor,independent_visited,native_lifts_visited,independent_total. No allocating String/Value formatting; the existing fixed serializer scratch envelope covers this temporary. Field values/counters remain u128 internally. Metered serialization charges quotes/digits normally; no budget or cap change.

ExactR_m goldens increase2bytes for quotedrowcursor.64bit m4..15:1194,1382,1570,1758,1946,2134,2322,2510,2698,2886,3074,3262.32bit:1124,1292,1460,1628,1796,1964,2132,2300,2468,2636,2804,2972. Fourteen group tests authored after adding typed string-deserialization roundtrip for0,2^64,2^90,u128MAX and rowcursorMAX, without serdeValue numeric/f64 conversion. No production decoder is introduced; central audit owns strict canonical decimal parsing/rejection tests.

rustfmt and git apply --check passed; no compiler/tests/execution/application. Groupv2semantic fixes and all remaining step4/centralverification limits remain unchanged. Root and independent wire auditor received immutablepatch.
