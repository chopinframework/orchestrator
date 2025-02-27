#!/bin/bash
# update_api_docs.sh - Script to update API documentation for utoipa OpenAPI generation

set -e

# Define colors for output
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Running cargo check to ensure utoipa annotations are valid...${NC}"
cargo check

echo -e "${GREEN}API documentation ready!${NC}"
echo -e "${YELLOW}API documentation locations:${NC}"
echo -e "  - ${GREEN}http://localhost:4001/swagger-ui/${NC} (Swagger UI - when server is running)"
echo -e "  - ${GREEN}http://localhost:4001/api-docs/openapi.json${NC} (Raw OpenAPI spec - when server is running)"

# Display startup instructions
echo -e "\n${YELLOW}To view the API documentation:${NC}"
echo -e "1. Start the server: ${GREEN}cargo run${NC}"
echo -e "2. Open ${GREEN}http://localhost:4001/swagger-ui/${NC} in your browser"

echo -e "\n${GREEN}utoipa integration is active - OpenAPI documentation will be auto-generated at runtime!${NC}"
echo -e "${YELLOW}Note: The OpenAPI spec is generated directly from the code annotations${NC}" 