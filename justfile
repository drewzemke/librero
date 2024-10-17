run:
    #!/usr/bin/env bash
    trap 'kill 0' SIGINT;
    (just run-client) & 
    (just run-server) & 
    wait

run-server:
    cd server && watchexec -c -w src -w Cargo.toml -r "cargo run --quiet"

run-client:
    cd client && pnpm run dev

api-gen:
    cd server && cargo run --bin api-gen
    cd client && pnpm run api-gen

e2e:
    - cd server && cargo run --bin e2e
