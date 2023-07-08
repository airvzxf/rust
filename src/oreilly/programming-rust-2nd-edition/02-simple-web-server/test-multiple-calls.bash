#!/usr/bin/env bash

for n in $(seq 1 15000);
do
    m=$((n*7))
    curl -X POST --data "n=${n}&m=${m}" http://localhost:3000/gcd &
done
