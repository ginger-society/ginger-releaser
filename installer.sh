#!/bin/bash

# Function to get the OS type
get_os() {
    case "$(uname -s)" in
        Linux*)     echo "unknown-linux-gnu";;
        Darwin*)    echo "apple-darwin";;
        CYGWIN*|MINGW*|MSYS*) echo "pc-windows-gnu";;
        *)          echo "unknown";;
    esac
}

# Function to get the CPU architecture
get_arch() {
    case "$(uname -m)" in
        x86_64)     echo "x86_64";;
        aarch64)    echo "aarch64";;
        arm64)      echo "aarch64";;
        *)          echo "unknown";;
    esac
}

# Get the architecture and OS
arch=$(get_arch)
os=$(get_os)

# Print the target string
if [[ "$arch" == "unknown" || "$os" == "unknown" ]]; then
    echo "Unsupported architecture or OS"
else
    echo "${arch}-${os}"
fi
