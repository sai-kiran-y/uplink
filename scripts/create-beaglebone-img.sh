#!/bin/bash

# To create tar of current rootfs
sudo find / -maxdepth 1 -mindepth 1 -not -type l -print0 | \
sudo tar -cvpzf /mnt/download/rootfs.tar.gz \
--exclude='/mnt/download/rootfs.tar.gz' \
--exclude='/uboot/*' \
--exclude='/proc/*' \
--exclude='/tmp/*' \
--exclude='/mnt/*' \
--exclude='/dev/*' \
--exclude='/sys/*' \
--exclude='/run/*' \
--exclude='/media/*' \
--exclude='/home/pi/*' / 
