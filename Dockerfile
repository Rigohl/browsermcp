# ============================================================================
# EXTREME BROWSER MCP - LIGHTWEIGHT DOCKER 🚀  
# Uses pre-compiled binary to avoid edition2024 build issues
# Build: docker build -t extreme-browser-mcp:latest .
# Run: docker run -d -p 8080:8080 --name extreme-browser-mcp extreme-browser-mcp:latest
# ============================================================================

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libsqlite3-0 \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && adduser --disabled-password --gecos '' mcpuser

# Setup application directory
WORKDIR /app
RUN mkdir -p /app/.mcp_backups/browser_data && \
    chown -R mcpuser:mcpuser /app

# Copy pre-compiled binary (compile locally first with: cargo build --release)
COPY target/release/ ./
RUN chmod +x browsermcp-server*
COPY mcp_config.json ./mcp_config.json

# Switch to non-root user
USER mcpuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Environment variables
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1
ENV MCP_HOST=0.0.0.0
ENV MCP_PORT=8080

# Start the server (try different binary names)
CMD ["sh", "-c", "./browsermcp-server* || ./browsermcp-server.exe"]
