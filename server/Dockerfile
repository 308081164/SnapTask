# ---- Build Stage ----
FROM node:20-alpine AS builder

WORKDIR /app

# Copy dependency manifests
COPY package.json ./

# Install all dependencies (including devDependencies for build)
RUN npm install

# Copy source code
COPY tsconfig.json ./
COPY src/ ./src/

# Build TypeScript
RUN npm run build

# ---- Production Stage ----
FROM node:20-alpine AS production

WORKDIR /app

# Install better-sqlite3 native dependencies
RUN apk add --no-cache python3 make g++

# Copy dependency manifests
COPY package.json ./

# Install production dependencies only
RUN npm install --omit=dev && npm cache clean --force

# Copy built JavaScript from builder stage
COPY --from=builder /app/dist ./dist

# Create data directory
RUN mkdir -p /app/data

# Create non-root user
RUN addgroup -g 1001 -S snaptask && \
    adduser -S snaptask -u 1001

# Set ownership
RUN chown -R snaptask:snaptask /app

USER snaptask

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:3000/api/health || exit 1

# Environment variables
ENV NODE_ENV=production
ENV PORT=3000
ENV DATABASE_PATH=/app/data/snaptask.db

# Start the server
CMD ["node", "dist/index.js"]
