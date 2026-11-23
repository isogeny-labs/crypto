struct Random {
    std::uint64_t state;
    std::uint64_t next() {
        std::uint64_t z = (state += 0x9e3779b97f4a7c15ULL);
        z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9ULL;
        z = (z ^ (z >> 27)) * 0x94d049bb133111ebULL;
        return z ^ (z >> 31);
    }
    int bounded(int bound) {
        auto limit = -std::uint64_t(bound) % bound;
        std::uint64_t value;
        do { value = next(); } while (value < limit);
        return value % bound;
    }
};
