# build client
FROM node:20-slim AS client-build

RUN npm i -g pnpm

WORKDIR /usr/src/app

COPY ./client/package.json ./client/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile --shamefully-hoist

COPY ./client .
RUN pnpm vite build


# build server
FROM rust:1.81 AS server-build

WORKDIR /usr/src/app

COPY ./server .

RUN cargo build --release

# deploy
FROM debian:stable AS deploy

WORKDIR /app

COPY --from=server-build /usr/src/app/target/release/librero-server .
COPY --from=client-build /usr/src/app/dist ./assets

EXPOSE 4000

CMD ["./librero-server"]
