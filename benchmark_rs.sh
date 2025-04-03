#!/usr/bin/env bash
cat ${1} | ./target/release/bunyan
cat ${1} | ./target/release/bunyan -o short
cat ${1} | ./target/release/bunyan -o json-10
cat ${1} | ./target/release/bunyan -o bunyan
