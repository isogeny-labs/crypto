"""Independently replay a reduced-state SHA-256 Joux CLI certificate."""
import hashlib
import json
import struct
import sys
from pathlib import Path

MASK = 0xffffffff
K = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
]
IV = (0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
      0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19)

def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)

def rotr(x, n):
    return ((x >> n) | (x << (32 - n))) & MASK

def compress(state: bytes, block: bytes) -> bytes:
    require(len(state) == 32 and len(block) == 64, "SHA-256 compression requires a 32-byte state and 64-byte block")
    words = list(struct.unpack('>8I', state))
    w = list(struct.unpack('>16I', block))
    for i in range(16, 64):
        x, y = w[i - 15], w[i - 2]
        s0 = rotr(x, 7) ^ rotr(x, 18) ^ (x >> 3)
        s1 = rotr(y, 17) ^ rotr(y, 19) ^ (y >> 10)
        w.append((w[i - 16] + s0 + w[i - 7] + s1) & MASK)
    a, b, c, d, e, f, g, h = words
    for i in range(64):
        s1 = rotr(e, 6) ^ rotr(e, 11) ^ rotr(e, 25)
        ch = (e & f) ^ (~e & g)
        t1 = (h + s1 + ch + K[i] + w[i]) & MASK
        s0 = rotr(a, 2) ^ rotr(a, 13) ^ rotr(a, 22)
        maj = (a & b) ^ (a & c) ^ (b & c)
        t2 = (s0 + maj) & MASK
        a, b, c, d, e, f, g, h = (t1 + t2) & MASK, a, b, c, (d + t1) & MASK, e, f, g
    return struct.pack('>8I', *((x + y) & MASK for x, y in zip(words, (a, b, c, d, e, f, g, h))))

def truncate(state: bytes, bits: int) -> bytes:
    n = (bits + 7) // 8
    head = bytearray(state[:n])
    if bits % 8:
        head[-1] &= 0xff >> (8 - bits % 8)
    return bytes(head) + bytes(len(state) - n)

# Self-check the separate compression implementation against a standard full digest.
message = b'abc'
padded = message + b'\x80' + bytes(56 - len(message) - 1) + (len(message) * 8).to_bytes(8, 'big')
require(compress(struct.pack('>8I', *IV), padded) == hashlib.sha256(message).digest(),
        "independent SHA-256 compression self-check failed")

if len(sys.argv) != 2:
    raise SystemExit("usage: verify_joux_sha256_20261025.py CERTIFICATE.json")
certificate = json.loads(Path(sys.argv[1]).read_text())
require(certificate['algorithm'] == 'SHA-256' and certificate['status'] == 'complete',
        "expected a completed SHA-256 certificate")
bits = certificate['truncate_bits']
initial = truncate(hashlib.sha256(b'').digest(), bits)
require(initial.hex() == certificate['initial_state_hex'], "initial reduced state mismatch")
state = initial
for index, link in enumerate(certificate['links']):
    left = bytes.fromhex(link['left_hex'])
    right = bytes.fromhex(link['right_hex'])
    require(left != right, f"link {index} uses identical blocks")
    a = truncate(compress(state, left), bits)
    b = truncate(compress(state, right), bits)
    require(a == b, f"link {index} failed independent compression replay")
    state = a
require(state.hex() == certificate['final_state_hex'], "final reduced state mismatch")
require(len(certificate['links']) == certificate['chain_length'], "chain length mismatch")
require(certificate['represented_paths'] == 1 << certificate['chain_length'],
        "represented path count mismatch")
print(json.dumps({'independent_sha256_self_check': True,
                  'certificate_links_replayed': len(certificate['links']),
                  'final_state_matches': True,
                  'represented_paths': certificate['represented_paths']}))
