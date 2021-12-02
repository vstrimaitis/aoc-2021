#!/bin/bash
set -e
if [[ $# -eq 0 ]] ; then
    echo "Usage: ./new_day.sh <day_number>"
    exit 1
fi

export DAY=$1
DAY_FORMATTED=$(printf "%02d" $DAY)
TEMPLATES_FOLDER="templates"
INPUT_FILE="inputs/$DAY_FORMATTED.txt"

aocd > $INPUT_FILE
echo "First few lines of the input:"
head $INPUT_FILE
echo "# of lines in the input: $(wc -l $INPUT_FILE)"

# Prepare Python solution
PYTHON_TARGET_FOLDER="python/$DAY_FORMATTED"
PYTHON_TEMPLATE_FOLDER="$TEMPLATES_FOLDER/python"

if [[ -d $PYTHON_TARGET_FOLDER ]] ; then
    read -p "Directory $PYTHON_TARGET_FOLDER already exists. Overwrite? (y/n) " yn
    case $yn in
        [Yy]* ) rm -rf $PYTHON_TARGET_FOLDER;;
        * ) echo "Exiting"; exit 0;;
    esac
fi

cp -r $PYTHON_TEMPLATE_FOLDER $PYTHON_TARGET_FOLDER

for filename in $PYTHON_TARGET_FOLDER/*; do
    tmpfile=$(mktemp)
    envsubst < $filename > $tmpfile
    mv $tmpfile $filename
done

# Prepare Rust solution
RUST_TEMPLATE_DAY_FILE="$TEMPLATES_FOLDER/rust/day_template.rs"
RUST_TARGET_DAY_FILE="rust/src/day_$DAY_FORMATTED.rs"
RUST_MAIN_FILE="rust/src/main.rs"

if [[ -d RUST_TARGET_DAY_FILE ]] ; then
    read -p "File $RUST_TARGET_DAY_FILE already exists. Overwrite? (y/n) " yn
    case $yn in
        [Yy]* ) rm $RUST_TARGET_DAY_FILE;;
        * ) echo "Exiting"; exit 0;;
    esac
fi

cp $RUST_TEMPLATE_DAY_FILE $RUST_TARGET_DAY_FILE
gsed -i "s#// !include modules#mod day_$DAY_FORMATTED;\n// !include modules#" $RUST_MAIN_FILE
gsed -i "s#// !include mapping#$DAY => Some(day_$DAY_FORMATTED::solve),\n        // !include mapping#" $RUST_MAIN_FILE
