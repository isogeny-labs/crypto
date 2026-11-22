# Independent simplicity-witness replay: pass

The once-only replay at pushed checkpoint `7903d33ad45` exited 0. The standalone verifier checked every one of the 296 positive modular witness records, retained all 85 unresolved records, and confirmed all 127 candidate exponents for each of the two declared absolute models.

The producer computes power characteristic polynomials through Newton traces and uses native polynomial gcd. The verifier imports none of that code. It computes determinant values of tI−A^m, interpolates the degree-eight characteristic polynomial, and checks squarefreeness with its own Euclidean polynomial arithmetic and Fermat inverses. Its independently computed totient sieve covers the entire finite candidate set. Positive, collision and exact even-polynomial controls passed.

This supports the stated finite certificate for the two F17 models under THEORY.md. It independently replays modular arithmetic, not the extension-field point counts or all underlying arithmetic. It is not proof-assistant verification, scientific admission, an index-calculus solve or a speedup claim.

Both original and replay raw hashes verified. No output overwrite, retry, new model, threshold change or witness selection occurred. Git custody replaces unavailable S3; no upload is claimed. The broader tenfold genus-2+ objective remains open.
