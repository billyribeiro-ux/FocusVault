#!/bin/bash
set -euo pipefail

# Generate TypeScript types from the OpenAPI spec
# Requires: openapi-typescript (npm install -g openapi-typescript)
# and a running server (or saved spec file)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SPEC_URL="${1:-http://localhost:3000/api/docs/openapi.json}"
OUTPUT_DIR="$ROOT_DIR/apps/web/src/lib/api/generated"

mkdir -p "$OUTPUT_DIR"

echo "Fetching OpenAPI spec from $SPEC_URL..."
npx openapi-typescript "$SPEC_URL" -o "$OUTPUT_DIR/api-types.ts"

echo "Types generated at $OUTPUT_DIR/api-types.ts"
