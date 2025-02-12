#!/bin/bash

ENV_TYPE=$1
VERSION=$2
OUTPUT_DIR="distributions/dockerhub/generated"

# Default versions
NODE_VERSION="22.12.0"
GO_VERSION="1.22.4"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Environment-specific configurations
case "$ENV_TYPE" in
  "pipeline")
    ENV_SPECIFIC_INSTALLS="RUN apt-get install -y make"
    ;;
  "dev")
    ENV_SPECIFIC_INSTALLS=$(cat <<-EOF
      RUN apt-get install -y make vim
      RUN npm install -g nodemon
EOF
    )
    ;;
  "beta"|"latest")
    ENV_SPECIFIC_INSTALLS=$(cat <<-EOF
      RUN apt-get install -y make vim postgresql-client
      RUN npm install -g nodemon typescript
EOF
    )
    ;;
  *)
    echo "Unknown environment type: $ENV_TYPE"
    exit 1
    ;;
esac

# Generate Dockerfile from template
sed \
  -e "s/\${VERSION}/$VERSION/g" \
  -e "s/\${ENV_TYPE}/$ENV_TYPE/g" \
  -e "s/\${NODE_VERSION}/$NODE_VERSION/g" \
  -e "s/\${GO_VERSION}/$GO_VERSION/g" \
  -e "s|\${ENV_SPECIFIC_INSTALLS}|$ENV_SPECIFIC_INSTALLS|g" \
  distributions/dockerhub/Dockerfile.template > "$OUTPUT_DIR/Dockerfile.$ENV_TYPE"

echo "Generated Dockerfile for $ENV_TYPE environment at $OUTPUT_DIR/Dockerfile.$ENV_TYPE"
