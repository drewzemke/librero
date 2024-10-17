#!/usr/bin/env fish

# go to the repo and check for changes
echo \n(date +"%a %b %d, %Y %H:%M:%S")
cd /home/drew/dev/librero
git fetch origin main

if test (git rev-list HEAD..origin/main --count) -gt 0; or test "$argv[1]" = "--force" 
    echo "New changes found. Pulling and deploying..."
    git pull origin main

    # TODO other checks?
    echo "Running checks..."
    cd server
    cargo test

    echo "Starting server in Docker..."
    cd ..
    docker build -t librero .
    docker compose -f /home/drew/services/docker-compose.yaml up -d

    echo "All done!"
else 
    echo "No new changes, nothing to do"
end
