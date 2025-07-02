#!/usr/bin/env python3

import os
import subprocess
import sys

FLIST_FILE = "flist"
STATE_FILE = ".run_state.txt"
SCRIPT_NAME = os.path.basename(__file__)

# Step 1: Load run count
if os.path.exists(STATE_FILE):
    with open(STATE_FILE, "r") as f:
        run_count = int(f.read().strip())
else:
    run_count = 0

# Step 2: Load all targets from flist
if not os.path.exists(FLIST_FILE):
    print("Error: flist file not found!")
    exit(1)

with open(FLIST_FILE, "r") as f:
    raw_targets = [line.strip() for line in f if line.strip()]

# Step 3: Separate normal and _q/_d targets
seen_prefixed = set()
final_targets = []

for target in raw_targets:
    if target.endswith("_q") or target.endswith("_d"):
        prefix = target[:-2]
        if prefix not in seen_prefixed:
            seen_prefixed.add(prefix)
            suffix = "_d" if run_count % 2 == 0 else "_q"
            final_targets.append(prefix + suffix)
    else:
        final_targets.append(target)

# Step 4: Run make on selected targets
for target in final_targets:
    print(f"Running: make {target}")
    subprocess.run(["make", target])

# Step 5: Print known file outputs
def print_file(name):
    print(f"\nContents of file '{name}':")
    try:
        with open(name, "r") as f:
            print(f.read())
    except FileNotFoundError:
        print(f"(file '{name}' not found)")

for fname in ["a_d", "a_q", "b", "c"]:
    print_file(fname)

# Step 6: Save incremented run count
with open(STATE_FILE, "w") as f:
    f.write(str(run_count + 1))

# Step 7: Print contents of all other files in current directory
print("\n📄 Other file contents:")

excluded = {FLIST_FILE, "Makefile", SCRIPT_NAME}

for fname in os.listdir("."):
    if os.path.isfile(fname) and fname not in excluded:
        print_file(fname)

