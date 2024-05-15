#!/bin/bash

# Name of the folder where files will be copied for each commit
output_folder="output"

# Create the output folder if it doesn't exist
mkdir -p $output_folder

# Counter for incrementing folder names
counter=0

# Iterate through each commit in the Git history
git log --reverse --format="%H" | while read commit_hash; do
    # Create a folder for each commit
    commit_folder="$output_folder/$counter"
    mkdir -p $commit_folder

    # Checkout the files of the commit into the corresponding folder
    git clone . $commit_folder

    # jump into our commit folder
    pushd $commit_folder

    # Reset the working directory to the original state
    git checkout $commit_hash

    # Remove git from the directory, treating the files as plain
    rm -rf .git

    # Go back to our main directory
    popd

    # Increment the counter for the next folder
    ((counter++))
done

echo "Files copied for each commit in the commit history."
