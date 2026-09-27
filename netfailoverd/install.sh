#!/bin/sh
# Install netfailoverd as a root LaunchDaemon in dry run mode first.
# Usage: ./install.sh
# Live switch requires setting dry_run = false in /usr/local/etc/netfailover.conf
set -eu
cd "$(dirname "$0")"
cargo build --release
sudo install -m 755 target/release/netfailoverd /usr/local/bin/netfailoverd
sudo install -m 755 target/release/netfailoverctl /usr/local/bin/netfailoverctl
sudo mkdir -p /usr/local/etc
if [ ! -f /usr/local/etc/netfailover.conf ]; then
  sudo install -m 644 config/example-netfailover.toml /usr/local/etc/netfailover.conf
  echo "installed default config"
else
  echo "config exists, leaving /usr/local/etc/netfailover.conf untouched"
fi
sudo install -m 644 launchd/org.seamless-failover.netfailoverd.plist /Library/LaunchDaemons/org.seamless-failover.netfailoverd.plist
sudo launchctl bootout system /Library/LaunchDaemons/org.seamless-failover.netfailoverd.plist 2>/dev/null || true
sudo launchctl bootstrap system /Library/LaunchDaemons/org.seamless-failover.netfailoverd.plist
sleep 2
sudo launchctl print system/org.seamless-failover.netfailoverd | head -20 || true
echo "installed. check status with: sudo /usr/local/bin/netfailoverctl --state /var/run/netfailover.state status"
