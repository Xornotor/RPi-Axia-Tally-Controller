#!/bin/bash

readonly TARGET_HOST=tallyctrl@10.216.1.80
readonly TARGET_PATH=/home/tallyctrl/
readonly TARGET_ARCH=aarch64-unknown-linux-gnu
readonly SOURCE_PATH_BIN=./target/${TARGET_ARCH}/release/rpi-iqx-tally-core
readonly SOURCE_PATH_WWW=./html/*

cargo build --release --target=aarch64-unknown-linux-gnu
sshpass -p tallyctrl rsync ${SOURCE_PATH_BIN} ${TARGET_HOST}:${TARGET_PATH}
sshpass -p tallyctrl rsync ${SOURCE_PATH_WWW} ${TARGET_HOST}:${TARGET_PATH}/html
sshpass -p tallyctrl ssh -t ${TARGET_HOST} "sudo mv /home/tallyctrl/rpi-iqx-tally-core /app && \
                                           sudo mv /home/tallyctrl/html/* /var/www/html && \
                                           sudo rm /app/tally_config.json && \
                                           sudo rm -r /home/tallyctrl/* && \
                                           sudo pkill screen && \
                                           cd /app && \
                                           sudo screen -d -m ./rpi-iqx-tally-core"
#sshpass -p tallyctrl ssh -t ${TARGET_HOST} sudo reboot