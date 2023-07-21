#!/bin/bash

read -p "Would you like to uninstall Uni-Sync? [N/y]: " -n 1 -r
echo 
if [[ $REPLY =~ ^[Yy]$ ]]
then
    sudo systemctl disable uni-sync
    sudo rm /etc/systemd/system/uni-sync.service
    cp -f /usr/sbin/uni-sync.json ./uni-sync-backup.json
    sudo rm /usr/sbin/uni-sync.json
    sudo rm /usr/sbin/uni-sync
fi
echo 'Removed Uni-Sync'
