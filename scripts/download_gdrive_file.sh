#!/bin/bash

# We exit on any error, since this script is also run in the ci/cd
set -e

########################################################################################################################
# Script constants
########################################################################################################################

# Temporary path to store the cookie file used for anonymous authentication against
# google drive.
declare cookie_file="/tmp/d_cookie"

# Static url to request file downloads from google drive.
declare d_download_url="https://drive.google.com/uc?export=download"

########################################################################################################################
# Script variables
########################################################################################################################

declare d_item_id
declare d_cookie

########################################################################################################################
# Functions
########################################################################################################################

get_item_id() {
  echo "$1" | grep -E -o "(\w|-){26,}"
}

get_download_cookie() {
  curl -sc "$cookie_file" "$d_download_url&id=$1" > /dev/null
  awk '/_warning_/ {print $NF}' "$cookie_file"
}

########################################################################################################################
# Main
########################################################################################################################

if [ -z "$1" ]; then
  echo "No download url passed! Aborting!"
  exit 1
fi

d_item_id=$(get_item_id "$1")
d_cookie=$(get_download_cookie "$d_item_id")

echo -e "Downloading $1..."
curl --insecure -C - -LOJb "$cookie_file" "$d_download_url&confirm=$d_cookie&id=$d_item_id"



