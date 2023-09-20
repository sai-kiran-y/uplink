# Getting one_time_setup.sh
# curl  --proto '=https' --tlsv1.2 -sSf one_time_setup.sh https://raw.githubusercontent.com/sai-kiran-y/uplink/test-rpi/examples/rpi/one_time_setup.sh | bash 

# get update_fstab.sh
curl --proto '=https' --tlsv1.2 -sSf -o update_fstab.sh https://raw.githubusercontent.com/sai-kiran-y/uplink/test-rpi/scripts/update_fstab.sh
chmod +x ./update_fstab.sh
./update_fstab.sh
mount -a
rm update_fstab.sh

# get uplink binary
if [ `cat /etc/hostname` == "beaglebone" ];
then
	curl --proto '=https' --tlsv1.2 -sSfL -o /usr/local/share/bytebeam/uplink https://github.com/bytebeamio/uplink/releases/download/v2.7.1/uplink-armv7-unknown-linux-gnueabihf
fi
	
if [ `cat /etc/hostname` == "raspberrypi" ];
then
	curl --proto '=https' --tlsv1.2 -sSfL -o /usr/local/share/bytebeam/uplink https://github.com/bytebeamio/uplink/releases/download/v2.7.1/uplink-aarch64-unknown-linux-gnu
fi

# get systemd script
mkdir -pv /mnt/download/systemd
curl --proto '=https' --tlsv1.2 -sSf -o /mnt/download/systemd/systemd.sh https://raw.githubusercontent.com/sai-kiran-y/uplink/test-rpi/scripts/systemd/systemd.sh

# get uplink.service
curl --proto '=https' --tlsv1.2 -sSf -o /mnt/download/systemd/uplink.service https://raw.githubusercontent.com/sai-kiran-y/uplink/test-rpi/scripts/systemd/uplink.service

# get check-root-partition.service
curl --proto '=https' --tlsv1.2 -sSf -o /mnt/download/systemd/check-root-partition.service https://raw.githubusercontent.com/sai-kiran-y/uplink/test-rpi/scripts/systemd/check-root-partition.service

# get config.toml 
curl --proto '=https' --tlsv1.2 -sSf -o /usr/local/share/bytebeam/config.toml https://raw.githubusercontent.com/sai-kiran-y/uplink/test-rpi/scripts/config.toml

# get check_root_part.sh
curl --proto '=https' --tlsv1.2 -sSf -o /mnt/download/check_root_part.sh https://raw.githubusercontent.com/sai-kiran-y/uplink/test-rpi/scripts/check_root_part.sh

# Install netcat and vim
sudo apt update 
sudo apt install vim -y
sudo apt install netcat -y

# Make uplink executable
chmod +x /usr/local/share/bytebeam/uplink
chmod +x /mnt/download/check_root_part.sh

cp /mnt/download/systemd/uplink.service /etc/systemd/system/
cp /mnt/download/systemd/check-root-partition.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable check-root-partition.service
systemctl start check-root-partition.service

# Check if boot part is mounted at /boot or /uboot 
BOOT=`df -h | grep "boot\|uboot" | awk '{print $NF}'`
touch $BOOT/two
touch /mnt/download/two
echo "Done!!! Place device.json in /mnt/download folder"
