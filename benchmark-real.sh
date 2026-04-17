#!/bin/bash
# Real-world benchmark with actual npm packages
# Run this to create and delete real node_modules

set -e

TEST_DIR="./benchmark-test"
RUST_BIN="./target/release/dnm"
JS_CLI="./bin/cli.js"

echo "================================================"
echo "  Real node_modules Performance Test"
echo "================================================"
echo ""

# Check prerequisites
echo "Checking prerequisites..."
if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found"
    exit 1
fi

if [ ! -f "$RUST_BIN" ]; then
    echo "Building Rust binary..."
    cargo build --release
fi

# Create test project with real dependencies
create_test_project() {
    local project_name=$1
    local num_deps=$2
    
    local proj_dir="$TEST_DIR/$project_name"
    mkdir -p "$proj_dir"
    
    cat > "$proj_dir/package.json" <<EOF
{
  "name": "$project_name",
  "version": "1.0.0",
  "dependencies": {}
}
EOF
    
    # Add real dependencies
    echo "Installing $num_deps packages..."
    cd "$proj_dir"
    
    # Use lightweight packages for faster testing
    for i in $(seq 1 $num_deps); do
        case $((i % 8)) in
            0) npm install --save lodash 2>/dev/null || true ;;
            1) npm install --save express 2>/dev/null || true ;;
            2) npm install --save axios 2>/dev/null || true ;;
            3) npm install --save moment 2>/dev/null || true ;;
            4) npm install --save chalk 2>/dev/null || true ;;
            5) npm install ---save debug 2>/dev/null || true ;;
            6) npm install --save uuid 2>/dev/null || true ;;
            7) npm install --save rimraf 2>/dev/null || true ;;
        esac
    done
    
    cd - > /dev/null
    
    # Count files
    local file_count=$(find "$proj_dir" -type f 2>/dev/null | wc -l)
    echo "  Created $file_count files"
}

# Measure deletion time
measure_deletion() {
    local tool=$1
    local path=$2
    local name=$3
    
    echo -n "Testing $name... "
    
    local start=$(date +%s.%N)
    
    if [ "$tool" = "rust" ]; then
        "$RUST_BIN" --no-safe "$path" > /dev/null 2>&1
    else
        node "$JS_CLI" --no-safe "$path" > /dev/null 2>&1
    fi
    
    local end=$(date +%s.%N)
    local elapsed=$(echo "$end - $start" | bc)
    
    printf "%6.2fs\n" $elapsed
    echo "$elapsed"
}

# Main test
echo ""
echo "Test 1: Small project (5 dependencies)"
echo "----------------------------------------"
create_test_project "small" 5

JS_TIME=$(measure_deletion "js" "$TEST_DIR/small" "JS")
create_test_project "small" 5
RUST_TIME=$(measure_deletion "rust" "$TEST_DIR/small" "Rust")

echo "JS: ${JS_TIME}s, Rust: ${RUST_TIME}s"
echo ""

echo "Test 2: Medium project (20 dependencies)"
echo "----------------------------------------"
create_test_project "medium" 20

JS_TIME=$(measure_deletion "js" "$TEST_DIR/medium" "JS")
create_test_project "medium" 20
RUST_TIME=$(measure_deletion "rust" "$TEST_DIR/medium" "Rust")

echo "JS: ${JS_TIME}s, Rust: ${RUST_TIME}s"
echo ""

echo "Test 3: Large project (50 dependencies)"
echo "----------------------------------------"
create_test_project "large" 50

JS_TIME=$(measure_deletion "js" "$TEST_DIR/large" "JS")
create_test_project "large" 50
RUST_TIME=$(measure_deletion "rust" "$TEST_DIR/large" "Rust")

echo "JS: ${JS_TIME}s, Rust: ${RUST_TIME}s"
echo ""

# Cleanup
echo "Cleaning up..."
rm -rf "$TEST_DIR"

echo "✓ Done"
