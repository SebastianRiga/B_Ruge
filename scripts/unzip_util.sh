#!/usr/bin/env bash

if [ -z "$1" ]; then
  echo "Path to archive missing! Aborting!"
fi

unzip -o "$1"

rm -rf "$1"