#!/bin/sh
# Remove the simulated outage.
# Usage: sudo ./pf-unblock.sh
set -eu
ANCHOR="netfailover.block"
sudo pfctl -a "$ANCHOR" -F all 2>/dev/null || true
echo "upstream block removed from anchor $ANCHOR"
