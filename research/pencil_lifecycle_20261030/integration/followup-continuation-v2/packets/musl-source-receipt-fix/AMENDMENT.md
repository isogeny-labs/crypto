# Additive receipt correction

The independent prospective review found that the new source-root assertion ran outside the failure-receipt scope. Move that same assertion to the first statement inside the supervised try block. Source, cache, compiler and timeout gates are unchanged. Prior proposal remains unchanged. No build or checkout is admitted by this correction.
