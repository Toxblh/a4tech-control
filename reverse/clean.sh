#!/bin/bash

# https://a4.rys.pw

if [[ -z ${1-} ]]; then
    echo "Don't provide a .pcapng file"
    exit 1
fi

filename=$1;
tcpdump -r "$filename" -x >> "$filename.txt";
sed -i s/"	0x0000:  "//g "$filename.txt";
sed -z -i s/"\n	0x00[a-f0-9][a-f0-9]:  "/" "/g "$filename.txt";
cat "$filename.txt" | xclip -i -selection c
