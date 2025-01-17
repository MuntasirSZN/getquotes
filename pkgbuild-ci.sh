#!/bin/bash

# Exit on any error
set -e

# Temporary directory to store downloaded files
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

# Fetch the latest Git tag and update pkgver
LATEST_TAG=$(git ls-remote --tags --sort=-v:refname https://github.com/MuntasirSZN/getquotes.git | grep -oP 'tags/\K.*' | head -n 1 | sed 's/^v//')
if [[ -n "$LATEST_TAG" ]]; then
	# Escape special characters for sed
	LATEST_TAG_ESC=$(echo "$LATEST_TAG" | sed 's/[\/&]/\\&/g')
	# Update the PKGBUILD
	sed -i "s/^pkgver=.*/pkgver=$LATEST_TAG_ESC/" PKGBUILD
	echo "Updated pkgver to $LATEST_TAG in PKGBUILD."
else
	echo "No Git tags found. pkgver was not updated."
	exit 1
fi

# Read the updated pkgver from PKGBUILD
PKGVER=$(sed -n 's/^pkgver=//p' PKGBUILD)

# Read source array from PKGBUILD
mapfile -t SOURCES < <(sed -n '/^source=(/,/^)/ { /^source=(/d; /^)/d; s/^[[:space:]]*"\(.*\)".*$/\1/; p }' PKGBUILD)

# Download files and calculate sha256sums
declare -A SUMS
for URL in "${SOURCES[@]}"; do
	FILE=$(basename "$URL")
	curl -L -o "$TMPDIR/$FILE" "$URL"
	SUM=$(sha256sum "$TMPDIR/$FILE" | awk '{print $1}')
	SUMS["$URL"]=$SUM
done

# Read existing sha256sums from PKGBUILD
mapfile -t OLD_SUMS < <(sed -n '/^sha256sums=(/,/^)/ { /^sha256sums=(/d; /^)/d; s/^[[:space:]]*"\(.*\)".*$/\1/; p }' PKGBUILD)

# Replace old sums with new sums
for i in "${!SOURCES[@]}"; do
	URL="${SOURCES[$i]}"
	NEW_SUM="${SUMS[$URL]}"
	OLD_SUM="${OLD_SUMS[$i]}"
	# Escape special characters for sed
	OLD_SUM_ESC=$(echo "$OLD_SUM" | sed 's/[\/&]/\\&/g')
	NEW_SUM_ESC=$(echo "$NEW_SUM" | sed 's/[\/&]/\\&/g')
	# Update the PKGBUILD
	sed -i "s/$OLD_SUM_ESC/$NEW_SUM_ESC/" PKGBUILD
done

echo "SHA256 sums and pkgver have been updated in PKGBUILD."
