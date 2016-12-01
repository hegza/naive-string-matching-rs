__kernel void naive_sm(
		__private int const m,
		__global char const* const text,
		__global char const* const pattern,
		__global int* const result)
{

	uint const idx = get_global_id(0);
	result[idx] = idx;

	for (int i = 0; i != m; ++i) {
		if (text[idx + i] != pattern[i]) {
			result[idx] = 0;
			return;
		}
	}

}
