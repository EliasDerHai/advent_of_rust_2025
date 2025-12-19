-- Automated test for DAP setup
-- Run with: nvim --headless -c "luafile test-dap.lua" -c "qa!"
-- Or inside nvim: :luafile test-dap.lua

local dap = require('dap')

print("=== Testing DAP Configuration ===\n")

-- Step 1: Configure adapter
print("Step 1: Configuring codelldb adapter...")
dap.adapters.codelldb = {
  type = 'server',
  port = "${port}",
  executable = {
    command = vim.fn.expand('~/.local/share/nvim/mason/packages/codelldb/extension/adapter/codelldb'),
    args = {"--port", "${port}"},
  }
}

-- Verify adapter exists
local adapter_path = dap.adapters.codelldb.executable.command
local adapter_exists = vim.fn.filereadable(adapter_path) == 1
print(string.format("  Adapter path: %s", adapter_path))
print(string.format("  Adapter exists: %s", adapter_exists and "✓ YES" or "✗ NO"))

if not adapter_exists then
  print("\n❌ ERROR: codelldb not found! Install with :MasonInstall codelldb")
  return
end

-- Step 2: Configure Rust debugging
print("\nStep 2: Configuring Rust debug configurations...")
local test_binary = vim.fn.getcwd() .. '/target/debug/deps/advent_of_rust_2025-da37a6dddf88d909'
local binary_exists = vim.fn.filereadable(test_binary) == 1

print(string.format("  Test binary: %s", test_binary))
print(string.format("  Binary exists: %s", binary_exists and "✓ YES" or "✗ NO"))

if not binary_exists then
  print("\n❌ ERROR: Test binary not found! Run: cargo test --no-run")
  return
end

dap.configurations.rust = {
  {
    name = "Test: Debug day 05 part 1",
    type = "codelldb",
    request = "launch",
    program = test_binary,
    args = {"day05::part1::tests::should_solve_day_05_part_01_sample", "--exact", "--nocapture"},
    cwd = '${workspaceFolder}',
    stopOnEntry = false,
  },
}

print(string.format("  Configurations loaded: %d", #dap.configurations.rust))

-- Step 3: Test connection
print("\nStep 3: Testing adapter connection...")
print("  Starting debug session...")

local success = false
local error_msg = nil

-- Set up a simple event listener to check if initialization works
dap.listeners.after.event_initialized["test"] = function()
  success = true
  print("  ✓ Debug adapter initialized successfully!")
  dap.listeners.after.event_initialized["test"] = nil

  -- Disconnect after successful init
  vim.defer_fn(function()
    pcall(function()
      dap.disconnect()
      dap.close()
    end)
  end, 100)
end

-- Capture errors
dap.listeners.after.disconnect["test"] = function()
  print("  ✓ Disconnected from adapter")
  dap.listeners.after.disconnect["test"] = nil
end

-- Try to start a session
local ok, err = pcall(function()
  -- This should trigger the adapter to start
  dap.run(dap.configurations.rust[1])
end)

if not ok then
  print("  ✗ Failed to start debug session:")
  print("    Error: " .. tostring(err))
  error_msg = err
else
  print("  ✓ Debug session started")
end

-- Wait a bit for initialization
print("\n  Waiting for initialization...")
vim.wait(5000, function()
  return success
end, 100)

print("\n=== Test Results ===")
if success then
  print("✓ DAP setup is working correctly!")
  print("\nYou can now debug Rust code:")
  print("  1. Open src/day05/part1.rs")
  print("  2. Set a breakpoint with <Leader>b on line 4")
  print("  3. Press <F5> to start debugging")
  print("  4. Select 'Test: Debug day 05 part 1'")
else
  print("✗ DAP setup test failed")
  if error_msg then
    print("Error: " .. tostring(error_msg))
  end
  print("\nTroubleshooting:")
  print("  1. Make sure codelldb is installed: :MasonInstall codelldb")
  print("  2. Rebuild tests: cargo test --no-run")
  print("  3. Check :messages for errors")
  print("  4. Check DAP log: :DapShowLog")
end

print("\n=== End of Test ===")
