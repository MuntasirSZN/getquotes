#!/bin/bash

# Configuration variables
REPO_OWNER="MuntasirSZN"
REPO_NAME="getquotes"
FORMULA_FILE="../packages/brew/getquotes.rb"
TEMPLATE_FILE="../packages/brew/getquotes.rb.template"
TEMP_DIR=$(mktemp -d)

# Function to fetch latest release information
fetch_latest_release() {
  response=$(curl -s https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest)
  if [ $? -ne 0 ]; then
    echo "Error fetching release information."
    exit 1
  fi
  if echo $response | jq -e '.message' >/dev/null; then
    error_message=$(echo $response | jq -r '.message')
    echo "API error: $error_message"
    exit 1
  fi
  version=$(echo $response | jq -r '.tag_name')
  assets_urls=$(echo $response | jq -r '.assets[].browser_download_url')
  echo "Version: $version"
  echo "Assets URLs: $assets_urls"
}

# Function to download assets
download_assets() {
  for url in $assets_urls; do
    filename=$(basename $url)
    echo "Downloading $url to ${TEMP_DIR}/${filename}"
    curl -L -o "${TEMP_DIR}/${filename}" $url
    if [ $? -ne 0 ]; then
      echo "Failed to download $url"
      exit 1
    fi
  done
}

# Function to calculate checksums and categorize
calculate_checksums() {
  declare -A assets_map

  for url in $assets_urls; do
    filename=$(basename $url)
    if [[ $filename == *aarch64-apple-darwin* ]]; then
      assets_map['MACOS_ARM_URL']=$url
      assets_map['MACOS_ARM_SHA']=$(sha256sum "${TEMP_DIR}/${filename}" | awk '{print $1}')
    elif [[ $filename == *x86_64-apple-darwin* ]]; then
      assets_map['MACOS_INTEL_URL']=$url
      assets_map['MACOS_INTEL_SHA']=$(sha256sum "${TEMP_DIR}/${filename}" | awk '{print $1}')
    elif [[ $filename == *aarch64-linux-android* ]]; then
      assets_map['LINUX_ARM_URL']=$url
      assets_map['LINUX_ARM_SHA']=$(sha256sum "${TEMP_DIR}/${filename}" | awk '{print $1}')
    elif [[ $filename == *x86_64-unknown-linux-gnu* ]]; then
      assets_map['LINUX_INTEL_URL']=$url
      assets_map['LINUX_INTEL_SHA']=$(sha256sum "${TEMP_DIR}/${filename}" | awk '{print $1}')
    fi
  done

  # Export variables for sed substitution
  export VERSION="${version}"
  export MACOS_ARM_URL="${assets_map['MACOS_ARM_URL']}"
  export MACOS_ARM_SHA="${assets_map['MACOS_ARM_SHA']}"
  export MACOS_INTEL_URL="${assets_map['MACOS_INTEL_URL']}"
  export MACOS_INTEL_SHA="${assets_map['MACOS_INTEL_SHA']}"
  export LINUX_ARM_URL="${assets_map['LINUX_ARM_URL']}"
  export LINUX_ARM_SHA="${assets_map['LINUX_ARM_SHA']}"
  export LINUX_INTEL_URL="${assets_map['LINUX_INTEL_URL']}"
  export LINUX_INTEL_SHA="${assets_map['LINUX_INTEL_SHA']}"
}

# Function to update the formula
update_formula() {
  # Check if all required variables are set
  if [ -z "${MACOS_ARM_URL}" ] || [ -z "${MACOS_ARM_SHA}" ] || [ -z "${MACOS_INTEL_URL}" ] || [ -z "${MACOS_INTEL_SHA}" ] || [ -z "${LINUX_ARM_URL}" ] || [ -z "${LINUX_ARM_SHA}" ] || [ -z "${LINUX_INTEL_URL}" ] || [ -z "${LINUX_INTEL_SHA}" ]; then
    echo "Missing required assets for the formula."
    exit 1
  fi

  # Replace placeholders in the template
  sed -e "s|{{VERSION}}|${VERSION}|g" \
    -e "s|{{MACOS_ARM_URL}}|${MACOS_ARM_URL}|g" \
    -e "s|{{MACOS_ARM_SHA}}|${MACOS_ARM_SHA}|g" \
    -e "s|{{MACOS_INTEL_URL}}|${MACOS_INTEL_URL}|g" \
    -e "s|{{MACOS_INTEL_SHA}}|${MACOS_INTEL_SHA}|g" \
    -e "s|{{LINUX_ARM_URL}}|${LINUX_ARM_URL}|g" \
    -e "s|{{LINUX_ARM_SHA}}|${LINUX_ARM_SHA}|g" \
    -e "s|{{LINUX_INTEL_URL}}|${LINUX_INTEL_URL}|g" \
    -e "s|{{LINUX_INTEL_SHA}}|${LINUX_INTEL_SHA}|g" \
    "${TEMPLATE_FILE}" >"${FORMULA_FILE}"
  echo "Formula updated to version ${version}."
}

# Main script execution
fetch_latest_release
download_assets
calculate_checksums
update_formula
rm -rf "${TEMP_DIR}"
