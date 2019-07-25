#!/usr/bin/env bash

set -exu

# rsync -aP ./target/x86_64-unknown-linux-musl/release/xdxd-backup root@docker-vm1.lan:/root/xdxd-backup/xdxd-backup
rsync -aP ./target/x86_64-unknown-linux-musl/release/xdxd-backup /home/hristo/xdxd-backup/xdxd-backup
