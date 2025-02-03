#!/bin/bash

# Check if the name is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <name>"
    exit 1
fi

PLUGIN_NAME=$1
RIG_NAME=$1
EXAMPLE_NAME=$1

# Create solagent-plugins
cargo new --lib solagent-plugins/$PLUGIN_NAME
# Rename package name in Cargo.toml
sed -i "s/name = \"$PLUGIN_NAME\"/name = \"solagent-plugin-$PLUGIN_NAME\"/" solagent-plugins/$PLUGIN_NAME/Cargo.toml

# Create rig-based tool
cargo new --lib solagent-adapters/rig/$RIG_NAME
# Rename package name in Cargo.toml
sed -i "s/name = \"$RIG_NAME\"/name = \"solagent-rig-$RIG_NAME\"/" solagent-adapters/rig/$RIG_NAME/Cargo.toml

# Create examples
cargo new examples/$EXAMPLE_NAME