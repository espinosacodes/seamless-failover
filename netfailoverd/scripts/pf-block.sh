#!/bin/sh
# Block upstream for the primary network to simulate an outage.
# Usage: sudo ./pf-block.sh
set -eu
ANCHOR="netfailover.block"
RULE="block out quick proto tcp to any port 80"
echo "$RULE" | sudo pfctl -a "$ANCHOR" -f - 2>/dev/null || true
sudo pfctl -e 2>/dev/null || true
echo "upstream port 80 blocked under anchor $ANCHOR"
