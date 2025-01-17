#!/bin/bash

# Exit on any error
set -e

# Temporary directory to store downloaded files
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

# Read source array from PKGBUILD
mapfile -t SOURCES < <(sed -n '/^source(\(.*\))$/,/^)$/ {/^\(/d; /,.*\)$/d; s/^[[:space:]]*//; p}' PKGBUILD)

# Download files and calculate sha256sums
declare -A SUMS
for URL in "${SOURCES[@]}"; do
  FILE=$(basename "$URL")
  curl -L -o "$TMPDIR/$FILE" "$URL"
  SUM=$(sha256sum "$TMPDIR/$FILE" | awk '{print $1}')
  SUMS["$URL"]=$SUM
done

# Read existing sha256sums from PKGBUILD
mapfile -t OLD_SUMS < <(sed -n '/^sha256sums(\(.*\))$/,/^)$/ {/^\(/d; /,.*\)$/d; s/^[[:space:]]*//; p}' PKGBUILD)

# Replace old sums with new sums
for i in "${!SOURCES[@]}"; do
  URL="${SOURCES[$i]}"
  NEW_SUM="${SUMS[$URL]}"
  OLD_SUM="${OLD_SUMS[$i]}"
  # Escape special characters for sed
  OLD_SUM_ESC=$(echo "$OLD_SUM" | sed 's/[\/&]/\\&/g')
  NEW_SUM_ESC=$(echo "$NEW_SUM" | sed 's/[\/&]/\\&/g')
  # Update the PKGBUILD
  sed -i "s/$OLD_SUM_ESC/$NEW_SUM_ESC/g" PKGBUILD
done

echo "SHA256 sums have been updated in PKGBUILD."
