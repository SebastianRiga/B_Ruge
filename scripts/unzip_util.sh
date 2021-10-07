#!/bin/bash

set -e

if [ -z "$1" ]; then
  echo "Path to archive missing! Aborting!"
fi

unzip -o "$1"

rm -rf "$1"