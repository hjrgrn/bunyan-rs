#!/usr/bin/env bash
npx bunyan -p benchmark_logs.txt
npx bunyan -o short -p benchmark_logs.txt
npx bunyan -o json-10 -p benchmark_logs.txt
npx bunyan -o bunyan -p benchmark_logs.txt
