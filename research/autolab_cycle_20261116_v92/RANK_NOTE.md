# Exact compression of binary separator certificates

Let K = GF(2^n), represented in a polynomial basis with generator T. Let D span 1 through T^(d−1), and let a separator cell be u0 + H, where H spans 1 through T^(k−1). Assume d + k − 1 ≤ n.

For fixed y, the binary summation polynomial is

    S3(x,y,u) = (y+u)^2 x^2 + yu x + (yu)^2 + B.

Write u = u0 + Σ e_i T^i and x = Σ s_j T^j. The coefficient of the mixed Boolean monomial e_i s_j is

    T^(2i+2j) + y T^(i+j) = F_y(T^(i+j)),
    F_y(t) = t^2 + yt.

The map F_y is F2-linear. Every exponent from 0 through d+k−2 occurs as i+j, and no product wraps modulo the field polynomial. Therefore all kd mixed columns span exactly the image under F_y of the (d+k−1)-dimensional prefix space. Replacing them by d+k−1 columns preserves the entire span.

The remaining columns are d coefficients of the fixed part and k right-hand-side directions. Thus the certificate rank is at most

    d + (d+k−1) + k = 2d + 2k − 1.

For n=83, d=18 and k=18, the full list of 360 columns becomes 71 columns, with rank at most 71. For k=23, 455 columns become 81. A dual vector annihilating these columns while pairing nontrivially with the constant proves the entire cell empty. The rank bound alone does not prove emptiness.

The implementation verifies equal full and compressed spans, memberships and canonical dual witnesses. It does not establish a new general rootfinding algorithm or mathematical novelty. A 23-bit cell still leaves 2^60 possible high-bit cell positions for fixed y, so a global algorithm needs additional structure.

Two transfer limits matter. If both sources and the separator direction space contain 1 and B=1, the fully free-source Boolean expansion contains a triple monomial with coefficient 1. Independent-monomial linearization then always spans the constant and cannot reject the cell. Source guards or product consistency are necessary. Over a prime field Fp, a single field-valued linear-span test has dimension one, so any nonzero column fills it; the binary extension-field mechanism needs a different prime-field backend.
