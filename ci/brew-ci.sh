#!/bin/bash
set -euo pipefail

TAP_REPO_DIR="homebrew-programs"
FORMULA_FILE="Formula/g/getquotes.rb"

if [ ! -d "${TAP_REPO_DIR}" ]; then
	echo "Error: Tap repository directory '${TAP_REPO_DIR}' not found."
	exit 1
fi

cd "${TAP_REPO_DIR}"

# 1. Get the latest tag from the getquotes upstream repository.
echo "Fetching latest tag from getquotes repository..."
LATEST_TAG=$(git ls-remote --tags https://github.com/MuntasirSZN/getquotes.git |
	awk -F'/' '/refs\/tags\/v[0-9]+\.[0-9]+\.[0-9]+$/ {print $NF}' | sort -V | tail -n1)

if [ -z "$LATEST_TAG" ]; then
	echo "Error: Could not determine the latest tag."
	exit 1
fi

echo "Latest tag is: ${LATEST_TAG}"

# 2. Define the base URL and construct URLs for each platform.
BASE_URL="https://github.com/MuntasirSZN/getquotes/releases/download/${LATEST_TAG}"
URL_MAC_ARM="${BASE_URL}/getquotes-aarch64-apple-darwin.tar.gz"
URL_MAC_INTEL="${BASE_URL}/getquotes-x86_64-apple-darwin.tar.gz"
URL_LINUX_ARM="${BASE_URL}/getquotes-aarch64-unknown-linux-gnu.tar.gz"
URL_LINUX_INTEL="${BASE_URL}/getquotes-x86_64-unknown-linux-gnu.tar.gz"

echo "Calculating SHA256 for each tarball..."
SHA_MAC_ARM=$(curl -sL "${URL_MAC_ARM}" | shasum -a 256 | awk '{print $1}')
SHA_MAC_INTEL=$(curl -sL "${URL_MAC_INTEL}" | shasum -a 256 | awk '{print $1}')
SHA_LINUX_ARM=$(curl -sL "${URL_LINUX_ARM}" | shasum -a 256 | awk '{print $1}')
SHA_LINUX_INTEL=$(curl -sL "${URL_LINUX_INTEL}" | shasum -a 256 | awk '{print $1}')

echo "SHA256 (macOS ARM): $SHA_MAC_ARM"
echo "SHA256 (macOS Intel): $SHA_MAC_INTEL"
echo "SHA256 (Linux ARM): $SHA_LINUX_ARM"
echo "SHA256 (Linux Intel): $SHA_LINUX_INTEL"

# 3. Verify the formula file exists.
if [ ! -f "${FORMULA_FILE}" ]; then
	echo "Error: Formula file '${FORMULA_FILE}' not found!"
	exit 1
fi

# 4. Update the formula file:
echo "Updating version and checksums in ${FORMULA_FILE}..."

# Replace any occurrence of the old tag (assumed to be vX.Y.Z) with the new tag in URLs.
sed -i.bak -E "s|(releases/download/)(v[0-9]+\.[0-9]+\.[0-9]+)|\1${LATEST_TAG}|g" "${FORMULA_FILE}"
rm "${FORMULA_FILE}.bak"

# Update SHA256 for macOS ARM tarball.
sed -i.bak -E '/getquotes-aarch64-apple-darwin\.tar\.gz/ { n; s/sha256 "[^"]+"/sha256 "'"${SHA_MAC_ARM}"'"/ }' "${FORMULA_FILE}"
rm "${FORMULA_FILE}.bak"

# Update SHA256 for macOS Intel tarball.
sed -i.bak -E '/getquotes-x86_64-apple-darwin\.tar\.gz/ { n; s/sha256 "[^"]+"/sha256 "'"${SHA_MAC_INTEL}"'"/ }' "${FORMULA_FILE}"
rm "${FORMULA_FILE}.bak"

# Update SHA256 for Linux ARM tarball.
sed -i.bak -E '/getquotes-aarch64-unknown-linux-gnu\.tar\.gz/ { n; s/sha256 "[^"]+"/sha256 "'"${SHA_LINUX_ARM}"'"/ }' "${FORMULA_FILE}"
rm "${FORMULA_FILE}.bak"

# Update SHA256 for Linux Intel tarball.
sed -i.bak -E '/getquotes-x86_64-unknown-linux-gnu\.tar\.gz/ { n; s/sha256 "[^"]+"/sha256 "'"${SHA_LINUX_INTEL}"'"/ }' "${FORMULA_FILE}"
rm "${FORMULA_FILE}.bak"

# 5. Commit and push changes directly.
COMMIT_MSG="chore: update getquotes ${LATEST_TAG}"
echo "Committing changes: ${COMMIT_MSG}"
git config user.name github-actions[bot]
git config user.email 41898282+github-actions[bot]@users.noreply.github.com
git add "${FORMULA_FILE}"
git commit -m "${COMMIT_MSG}"

echo "Pushing changes directly to the repository..."
git push origin HEAD

echo "Done. The getquotes formula has been updated to ${LATEST_TAG} and pushed."
