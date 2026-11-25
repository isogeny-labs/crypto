#!/usr/bin/env python3
"""Run the frozen degree-32 replication with the source-pinned binary."""
import f4_degree32 as degree32

degree32.EXPECTED = "c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d"

import f4_degree32_replication as replication

if __name__ == "__main__":
    replication.main()
