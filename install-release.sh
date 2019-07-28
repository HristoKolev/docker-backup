#!/usr/bin/env bash

set -exu

rsync -aP ./target/x86_64-unknown-linux-musl/release/xdxd-backup root@docker-vm1.lan:/root/xdxd-backup/xdxd-backup
sudo rsync -aP ./target/x86_64-unknown-linux-musl/release/xdxd-backup /root/xdxd-backup/xdxd-backup
