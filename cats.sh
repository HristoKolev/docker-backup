#!/usr/bin/env bash

#set -exu

read my_var

echo $my_var

for i in 1 2 3 4 5; do
    sleep 1;
    echo "Welcome $i times"
done
