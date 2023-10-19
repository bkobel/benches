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

## Todo

- [ ] Add CPU and GPU hashing test: MD5, SHA1, SHA256, PBKDF2, Argon2
- [ ] Add crypto tests: AES256, RSA2048, RSA Elliptic (?)
- [ ] Add topology (Dijkstra) traversing tests
- [ ] Add compression tests (gzip)
- [ ] Think about heavy compilation tests, alternative to chromium which is unstable and hard to replicate across different platforms even through Docker :/=
