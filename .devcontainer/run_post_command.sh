#!/bin/bash
set -e

echo "Running post-create commands..."

# Update package list and install required packages
echo "Installing system packages..."
apt update
apt install -y protobuf-compiler libprotobuf-dev shellcheck

# Install protolint
echo "Installing protolint..."
bash scripts/install_protolint.sh

# Update and upgrade system packages
echo "Upgrading system packages..."
apt update
apt upgrade -y

# Install actionlint
echo "Installing actionlint..."
curl -sSfL https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash | bash
sudo mv actionlint /usr/local/bin/

# Conditionally install GitHub Actions Runner based on architecture
ARCH=$(uname -m)
echo "Detected architecture: $ARCH"

if [ "$ARCH" = "x86_64" ] || [ "$ARCH" = "amd64" ]; then
    echo "Installing GitHub Actions Runner for $ARCH..."
    # Add actual GitHub Actions Runner installation commands here if needed
    # For now, this is a placeholder since the feature was handling it before
    echo "GitHub Actions Runner installation would go here for compatible architecture"
else
    echo "Skipping GitHub Actions Runner installation on $ARCH (not supported on ARM/Raspberry Pi)"
fi

echo "Post-create commands completed successfully!"
