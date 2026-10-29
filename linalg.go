package ecdl

// LeftKernelVector returns a nonzero c with sum_j c_j * rows[j] = 0 (mod r),
// where each row is a sparse map from column to coefficient. It needs
// len(rows) > rank, which len(rows) > ncols guarantees; ok is false otherwise.
// r must be prime.
func LeftKernelVector(rows []map[int]uint64, ncols int, r uint64) (c []uint64, ok bool) {
	k := len(rows)
	// Augment each row with an identity block that records how it was formed.
	mat := make([][]uint64, k)
	for j, row := range rows {
		mat[j] = make([]uint64, ncols+k)
		for col, v := range row {
			mat[j][col] = v % r
		}
		mat[j][ncols+j] = 1
	}
	used := make([]bool, k)
	for col := 0; col < ncols; col++ {
		piv := -1
		for j := 0; j < k; j++ {
			if !used[j] && mat[j][col] != 0 {
				piv = j
				break
			}
		}
		if piv < 0 {
			continue
		}
		used[piv] = true
		inv := invMod(mat[piv][col], r)
		for t := range mat[piv] {
			mat[piv][t] = mulMod(mat[piv][t], inv, r)
		}
		for j := 0; j < k; j++ {
			if used[j] || mat[j][col] == 0 {
				continue
			}
			f := mat[j][col]
			for t := range mat[j] {
				if mat[piv][t] != 0 {
					mat[j][t] = subMod(mat[j][t], mulMod(f, mat[piv][t], r), r)
				}
			}
		}
	}
	// An unused row now has zeros in every column; its identity block is a
	// dependency, and it is nonzero because the row's own entry stays 1.
	for j := 0; j < k; j++ {
		if !used[j] {
			return mat[j][ncols:], true
		}
	}
	return nil, false
}
