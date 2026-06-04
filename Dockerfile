FROM node:22-bookworm AS frontend-build

WORKDIR /app
COPY frontend/ .
RUN npm install -g pnpm && pnpm install && pnpm build

FROM rust:1-bookworm AS backend-build

WORKDIR /app
COPY backend/ .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app
COPY --from=backend-build /app/target/release/bookmark-backend .
COPY --from=frontend-build /app/dist ./frontend/dist

ENV FRONTEND_DIR=frontend/dist

RUN mkdir -p .local
EXPOSE 8989

CMD ["./bookmark-backend"]
