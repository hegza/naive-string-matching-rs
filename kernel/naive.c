__kernel void naive_sm(
		__private int const m,
		__global char const* const text,
		__global char const* const pattern,
		__global int* const result
		/*__local float *sharedData*/)
{

	// __local float shared_data[32]; // shared between a 32 item workgroup

	uint const idx = get_global_id(0);
	result[idx] = idx;

	for (int i = 0; i != m; ++i) {
		if (text[idx + i] != pattern[i]) {
			result[idx] = 0;
			return;
		}
	}

}
