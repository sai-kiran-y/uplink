# Rootfs Update

This is used in situations, where one needs to update the entire system, including the
rootfs and kernel.  A typical way of doing this would be to take the
tar of the updater, rootfs_update.sh script and image that you want the system to be updated to. 
With the help of [this](https://github.com/bytebeamio/uplink/blob/main/scripts/create-beaglebone-img.sh) script,
tar of the rootfs can be created.

## Create the rootfs update tar file
The script make_firmware_update.sh creates the tar file rootfs_update.tar.gz from the 
new image, updater and rootfs_update.sh scripts

## Uploading to the Bytebeam cloud
Upload the file rootfs_update.tar.gz in the "Update Firmware" section of the Bytebeam
cloud. Once the OTA update is triggered, the file rootfs_update.tar.gz is downloaded to
the device and its contents are extracted. Then updater script is run, which inturn
calls rootfs_update.sh with appropriate parameters. Note that updater is like a wrapper
script to rootfs_update.sh 

## Working of rootfs_update.sh scripts
The rootfs_update.sh script extracts the rootfs to the inactive partition of the system
and replaces the necessary firmware files and kernel in the boot partition with the new
firmware files and new kernel respectively and reboots.

## Status of the update
The progress of the update can be seen in the Bytebeam cloud. If the 
firmware files and the kernel are proper, the system is rebooted to the new kernel.