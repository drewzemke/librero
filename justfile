run:
    cargo leptos watch & \
    tailwindcss -i style/tailwind-in.css -o style/tailwind-out.css --watch & \
    wait
