#!/bin/bash

read -p "Would you like to uninstall Uni-Sync? [N/y]: " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
	sudo systemctl disable uni-sync
	sudo rm /usr/lib/systemd/system/uni-sync.service
	cp -f /usr/local/bin/uni-sync.json ./uni-sync-backup.json
	sudo rm /usr/local/bin/uni-sync.json
	sudo rm /usr/local/bin/uni-sync
fi
echo 'Removed Uni-Sync'
