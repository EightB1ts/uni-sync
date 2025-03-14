#!/bin/bash

read -p "Would you like to uninstall Uni-Sync? [N/y]: " -n 1 -r
echo 
if [[ $REPLY =~ ^[Yy]$ ]]
then
    sudo systemctl disable uni-sync
    sudo rm /etc/systemd/system/uni-sync.service
    cp -f /etc/uni-sync/uni-sync.json ./uni-sync-backup.json
    sudo rm /etc/uni-sync/uni-sync.json
    sudo rm -r /etc/uni-sync
fi
echo 'Removed Uni-Sync'
