#!/usr/bin/env bash

########################################################################################################################
# Script variables
########################################################################################################################

# Naming schemas

declare resource_package_name="r"
declare resource_base_module="mod.rs"
declare resource_folder_name="resources"

# Folders

declare out_folder="$PWD/src/$resource_package_name"
declare resource_folder="$PWD/$resource_folder_name"

########################################################################################################################
# Functions
########################################################################################################################

create_bose_module() {
  declare modules=("$@")
  declare output_file="$out_folder/$resource_base_module"

  printf "//! Base resource module, bundling all resource sub-modules.\n\n" >> "$output_file"

  for mod in "${modules[@]}"
  do
    echo "pub mod $mod;" >> "$output_file"
  done
}

create_sub_module() {
  local module_name="$1"
  local output_file="$out_folder/$module_name.rs"
  local input_folder="$resource_folder/$module_name"

  readarray -t files <<< "$(find "$input_folder" -mindepth 1 -maxdepth 1 -type f -printf "%P\n")"

  echo "#![allow(dead_code)]" >> "$output_file"
  printf "//! Module for %s resource files.\n\n" "$module_name" >> "$output_file"

  for file in "${files[@]}"
  do
    local variable_name
    variable_name="$(echo "$file" | sed -e 's/ /_/;s/-/_/;s/\./_/')"

    echo "/// $file" >> "$output_file"
    printf "pub const %s: &str = \"%s\";\n\n" "${variable_name^^}" "$resource_folder_name/$module_name/$file" >> "$output_file"
  done
}

clean() {
  echo "> Cleaning generated resource_folder_name modules..."
  rm -rf "$out_folder"
  echo "> Done"
}

########################################################################################################################
# Main
########################################################################################################################

echo "> Staring project resource_folder_name indexing"
echo "> Using resource root at: $resource_folder"
echo "> Using output folder at: $out_folder"

clean

echo "> Creating resource package..."

mkdir "$out_folder"

echo "> Reading resource sub-modules..."

readarray -t dirs <<< "$(find "$resource_folder" -mindepth 1 -maxdepth 1 -type d -printf "%P\n")"

echo "> Found resource folders:"

for dir in "${dirs[@]}"
do
  echo "-> $dir"
done

echo "> Generating module files..."

create_bose_module "${dirs[@]}"

for dir in "${dirs[@]}"
do
  create_sub_module "$dir"
done



