# benches

## Run

``` ./execute.sh -f -m ```

## Results

| CPU Model    | Logical Cores | Fibonacci(47) t32 | Matrices(250) 2k*2k   |
|--------------|---------------|-------------------|-----------------------|
| 7700x        | 16            | 6.739226829s      | 68.590633101s         |
| i9-13900k    | 32            | 2.9828701s        | 64.4862996s           |
| M1 Pro 14"   | 10            | 11.7964125s       | 67.383976416s         |
| M2 Pro 14"   | 10            | 11.690845458s     | 72.149219041s         |
| i5 6gen 14"  | 4             | 66.340590962s     | 401.667953762s        |

## Results insights

Performance Variation Among CPUs:

- The CPUs with higher logical core counts (i9-13900k with 32 cores) performed better in the Fibonacci calculation compared to others, demonstrating the benefits of parallel processing in a multi-threaded computation task.
- In matrix multiplication, the 6th generation i5 significantly lagged behind other CPUs. This could be due to its lower core count (4 cores) and potentially older architecture affecting its ability to handle the large-scale matrix operations efficiently.
- The M2 Pro 14" performed slightly worse than its M1 Pro 14" counterpart in matrix multiplication, which is surprising given they have the same core count. This might need further investigation to understand the underlying causes.

### Summary:
The data showcases a clear hierarchy in performance among the different CPUs. The i9-13900k stands out with the fastest times in both tests, likely benefitting from its high core count and potentially superior single-thread performance. This is closely followed by the 7700x, which also demonstrates strong performance, albeit with fewer cores. The ARM-based M1 Pro and M2 Pro chips exhibit competitive performance despite their lower core count, hinting at efficient threading or other architectural advantages inherent to the ARM design. On the other end of the spectrum, the i5 6th gen significantly trails in performance in both tests. Its lower core count and possibly other factors like lesser memory bandwidth or cache sizes could be attributing to its lag in performance. This analysis underscores the impact of core count, architecture, and potentially other hardware aspects on computational performance across different tasks.

## Todo

- [ ] Add CPU and GPU hashing test: MD5, SHA1, SHA256, PBKDF2, Argon2
- [ ] Add crypto tests: AES256, RSA2048, RSA Elliptic (?)
- [ ] Add topology (Dijkstra) traversing tests
- [ ] Add compression tests (gzip)
- [ ] Think about heavy compilation tests, alternative to chromium which is unstable and hard to replicate across different platforms even through Docker :/=
