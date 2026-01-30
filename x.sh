#!/bin/bash

echo "🔧 Checking cotask installation..."
echo "-----------------------------------"

# 1) Check Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Install Rust first."
    exit 1
fi
echo "✅ Cargo found"

# 2) Build project
echo "📦 Building project..."
if cargo build --release &> /dev/null; then
    echo "✅ Build successful"
else
    echo "❌ Build failed"
    exit 1
fi

# 3) Install binary
echo "📥 Installing cotask..."
cargo install --path . --force &> /dev/null
echo "✅ Installation complete"

# 4) Check PATH
if echo "$PATH" | grep -q "$HOME/.cargo/bin"; then
    echo "✅ Cargo bin directory is in PATH"
else
    echo "⚠️  ~/.cargo/bin not in PATH"
    echo "Add this to ~/.zshrc or ~/.bashrc:"
    echo 'export PATH="$HOME/.cargo/bin:$PATH"'
fi

# 5) Verify command exists
if command -v cotask &> /dev/null; then
    echo "✅ cotask command found at: $(which cotask)"
else
    echo "❌ cotask command not found"
    exit 1
fi

# 6) Test run
echo "🚀 Running test command..."
cotask --help &> /dev/null

if [ $? -eq 0 ]; then
    echo "✅ cotask runs successfully"
else
    echo "❌ cotask failed to run"
    exit 1
fi

echo "-----------------------------------"
echo "🎉 Everything looks good!"
