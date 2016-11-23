__kernel void kmp(
		__private int const n,
		__private int const m,
		__global char const* const text,
		__global char const* const pattern,
		__global char const* const prefix,
		__global int* const result)
{

	uint const idx = get_global_id(0);

	uint q = 0;
	for (int i = idx; i != n+1; ++i) {
        while (q > 0 && pattern[q] != text[i]) {
            q = prefix[q];
        }
        if (pattern[q] == text[i]) {
            q += 1;
        }
        if (q == m) {
			result[idx] = i+1-m;
			return;
        }
	}

	result[idx] = 0;

}

