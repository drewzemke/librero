run:
    #!/usr/bin/env bash
    trap 'kill 0' SIGINT;
    (just run-client) & 
    (just run-server) & 
    wait

run-server:
    cd server && watchexec -c -w src -w Cargo.toml -r "cargo run --quiet"

run-client:
    cd client && npm run dev
    
