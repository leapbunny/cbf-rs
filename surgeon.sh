#!/bin/bash

CBF_PACKET_SIZE=16384
byte1=00

# Initialize file position
file_position=0

# Send data loop
while true; do
    # Construct SCSI Write(10) command
    sg_raw $1 -b -s $CBF_PACKET_SIZE -n -k $file_position -i "$2" 2A 00 00 00 00 $byte1 00 00 20 00

    # Check status
    # if [[ "$status" != "Good" ]]; then
    #    echo "Error: Status is not Good. Exiting."
    #    exit 1
    # fi

    # Increment file position
    file_position=$((file_position + CBF_PACKET_SIZE))

    # Check if end of file
    if [[ $(stat -c %s "$2") -le $file_position ]]; then
        break
    fi

    # Change byte1 to '01' after first pass
    byte1='01'
done

# Verify
sg_verify $1 
