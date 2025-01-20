#!/bin/bash

# Exit on any error
set -e

# Fetch the latest tag from the repository
LATEST_TAG=$(git ls-remote --tags https://github.com/MuntasirSZN/getquotes.git | grep -oP 'tags/v?\K([0-9]+(\.[0-9]+)+)' | sort -V | tail -n 1)

if [[ -z "$LATEST_TAG" ]]; then
	echo "No Git tags found. pkgver was not updated."
	exit 1
fi

# Read current pkgver from PKGBUILD
CURRENT_PKGVER=$(sed -n 's/^pkgver=//p' ../packages/aur/getquotes/PKGBUILD)

# Check if pkgver is already up to date
if [[ "$CURRENT_PKGVER" == "$LATEST_TAG" ]]; then
	echo "pkgver is already up to date: $CURRENT_PKGVER"
	exit 0
fi

# Update pkgver in PKGBUILD
sed -i "s/^pkgver=.*/pkgver=$LATEST_TAG/" ../packages/aur/getquotes/PKGBUILD

echo "Updated pkgver to $LATEST_TAG in PKGBUILD."
