run:
    cargo leptos watch & \
    tailwindcss -i style/tailwind-in.css -o style/tailwind-out.css --watch & \
    wait

e2e:
    cd e2e && deno run -A npm:playwright test
