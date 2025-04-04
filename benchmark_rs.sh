#!/usr/bin/env bash
./target/release/bunyan benchmark_logs.txt
./target/release/bunyan -o short benchmark_logs.txt
./target/release/bunyan -o json-10 benchmark_logs.txt
./target/release/bunyan -o bunyan benchmark_logs.txt
