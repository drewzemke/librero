#!/usr/bin/env fish

# go to the repo and check for changes
echo \n(date +"%a %b %d, %Y %H:%M:%S")
cd /home/drew/dev/librero
git fetch origin main

if test (git rev-list HEAD..origin/main --count) -gt 0; or test "$argv[1]" = "--force" 
    echo "New changes found. Pulling and deploying..."

    git pull origin main

    # build server
    # TODO other checks?
    cd server
    cargo test

    cd ..
    # TODO: integrate with docker-compose?
    docker build -t librero-server .
    docker stop librero-server
    docker rm librero-server
    docker run -d --name librero-server -p 4000:4000 librero-server

    echo "All done!"
else 
    echo "No new changes, nothing to do"
end
