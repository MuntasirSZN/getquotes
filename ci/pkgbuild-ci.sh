#!/bin/bash

# Exit on any error
set -e

# Temporary directory to store downloaded files
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

# Fetch the latest Git tag and update pkgver
LATEST_TAG=$(git ls-remote --tags https://github.com/MuntasirSZN/getquotes.git | grep -oP 'tags/v?\K([0-9]+(\.[0-9]+)+)' | sort -V | tail -n 1)

if [[ -z "$LATEST_TAG" ]]; then
  echo "No Git tags found. pkgver was not updated."
  exit 1
fi

# Escape special characters for sed
LATEST_TAG_ESC=$(echo "$LATEST_TAG" | sed 's/[\/&]/\\&/g')

# Update the PKGBUILD
sed -i "s/^pkgver=.*/pkgver=$LATEST_TAG_ESC/" ../packages/aur/getquotes/PKGBUILD
echo "Updated pkgver to $LATEST_TAG in PKGBUILD."

# Read the updated pkgver from PKGBUILD
PKGVER=$(sed -n 's/^pkgver=//p' ../packages/aur/getquotes/PKGBUILD)

# Read source array from PKGBUILD
mapfile -t SOURCES < <(sed -n '/^source=(/,/^)/ { /^source=(/d; /^)/d; /^[[:space:]]*#/d; s/^[[:space:]]*"\([^"]*\)".*$/\1/; p }' ../packages/aur/getquotes/PKGBUILD)

# Download files and calculate sha256sums
declare -A SUMS
for URL in "${SOURCES[@]}"; do
  FILE=$(basename "$URL")
  curl -L -o "$TMPDIR/$FILE" "$URL"
  SUM=$(sha256sum "$TMPDIR/$FILE" | awk '{print $1}')
  SUMS["$URL"]=$SUM
done

# Read existing sha256sums from PKGBUILD with quotes
mapfile -t OLD_SUMS_QUOTED < <(sed -n '/^sha256sums=(/,/^)/ { /^sha256sums=(/d; /^)/d; /^[[:space:]]*#/d; s/^[[:space:]]*"\([^"]*\)".*$/"\1"/; p }' ../packages/aur/getquotes/PKGBUILD)

# Replace old sums with new sums, preserving quotes
for i in "${!SOURCES[@]}"; do
  URL="${SOURCES[$i]}"
  NEW_SUM="${SUMS[$URL]}"
  OLD_SUM_QUOTED="${OLD_SUMS_QUOTED[$i]}"
  # Escape special characters for sed
  OLD_SUM_ESC=$(echo "$OLD_SUM_QUOTED" | sed 's/[\/&]/\\&/g')
  NEW_SUM_ESC=$(echo "\"$NEW_SUM\"" | sed 's/[\/&]/\\&/g')
  # Update the PKGBUILD
  sed -i "s/$OLD_SUM_ESC/$NEW_SUM_ESC/" ../packages/aur/getquotes/PKGBUILD
done

echo "SHA256 sums and pkgver have been updated in PKGBUILD."
