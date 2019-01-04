@ cargo test -- --nocapture > bench.txt && for /l %%i in (1, 1, %1)  do @ cargo bench >> bench.txt
