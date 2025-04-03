#!/usr/bin/env bash
cat $1 | npx bunyan
cat $1 | npx bunyan -o short
cat $1 | npx bunyan -o json-10
cat $1 | npx bunyan -o bunyan
