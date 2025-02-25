#!/bin/bash

file_paths=(
"./data/jane_profile_07272018.doc"
"./data/kwood_profile_04022017.doc"
"./data/pchow_profile_05152019.doc"
"./data/janez_profile_11042019.doc"
"./data/jane_pic_07282018.jpg"
"./data/kwood_pic_04032017.jpg"
"./data/pchow_pic_05162019.jpg"
"./data/jane_contact_07292018.csv"
"./data/kwood_contact_04042017.csv"
"./data/pchow_contact_05172019.csv"
)

for file_path in "${file_paths[@]}"; do
    dir_path=$(dirname "$file_path")
    mkdir -p "$dir_path"
    touch "$file_path"
    echo "Created file: $file_path"
done
