# Base image with pnpm enabled
FROM node:22-slim AS base
RUN apt update && apt install -y git
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
COPY . /app
WORKDIR /app

# Install production dependencies
FROM base AS prod-deps
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --prod --frozen-lockfile

# Install all dependencies and build
FROM base AS build
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

ARG PUBLIC_API_DOMAIN

ENV PUBLIC_API_DOMAIN=$PUBLIC_API_DOMAIN

RUN pnpm run build

# Final image
FROM oven/bun:slim
# Install curl for healthcheck
RUN apt update && apt install -y curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copy 'node_modules' from 'prod-deps' and 'build' stages
COPY --from=build /app/ /app/
ENV HOST=0.0.0.0
ENV PORT=4321

EXPOSE 4321

CMD ["bun", "dist/server/entry.mjs"]