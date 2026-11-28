import hashlib, random

def seed_of(label):
    return int.from_bytes(hashlib.sha256(label.encode()).digest()[:8], 'big')

def rng(label):
    return random.Random(seed_of(label))
