run:
    #!/usr/bin/env bash
    trap 'kill 0' SIGINT;
    (just run-client) & 
    (just run-server) & 
    wait

run-server:
    cd server && watchexec -c -w src -w Cargo.toml -r "cargo run --quiet"

run-client:
    cd client && deno task dev

api-gen:
    cd server && cargo run --bin api-gen
    cd client && deno task api-gen

e2e:
    - cd server && cargo test --features e2e_tests -- --nocapture
