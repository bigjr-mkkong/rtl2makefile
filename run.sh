#!/bin/bash

# Ensure the flist file exists
if [[ ! -f flist ]]; then
    echo "Error: flist file not found!"
    exit 1
fi

# Read flist line by line
while IFS= read -r target; do
    echo "Running: make $target"
    make "$target"
done < flist

# Display contents of a and b
echo -e "\nContents of file 'a':"
cat a 2>/dev/null || echo "(file 'a' not found)"

echo -e "\nContents of file 'b':"
cat b 2>/dev/null || echo "(file 'b' not found)"

