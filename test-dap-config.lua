-- Test DAP configuration for Rust/CodeLLDB
-- Source this file in Neovim with :luafile %

local dap = require('dap')

-- Configure codelldb adapter
dap.adapters.codelldb = {
  type = 'server',
  port = "${port}",
  executable = {
    -- Path to codelldb installed via Mason
    command = vim.fn.expand('~/.local/share/nvim/mason/packages/codelldb/extension/adapter/codelldb'),
    args = {"--port", "${port}"},
  }
}

-- Configure Rust debugging
dap.configurations.rust = {
  {
    name = "Launch test binary",
    type = "codelldb",
    request = "launch",
    program = function()
      -- Return the test binary path
      return vim.fn.getcwd() .. '/target/debug/deps/advent_of_rust_2025-da37a6dddf88d909'
    end,
    cwd = '${workspaceFolder}',
    stopOnEntry = false,
    args = {},
  },
  {
    name = "Debug day 05 part 1 sample",
    type = "codelldb",
    request = "launch",
    program = vim.fn.getcwd() .. '/target/debug/deps/advent_of_rust_2025-da37a6dddf88d909',
    args = {"day05::part1::tests::should_solve_day_05_part_01_sample", "--exact", "--nocapture"},
    cwd = '${workspaceFolder}',
    stopOnEntry = false,
  },
}

-- Test if adapter is configured
print("DAP adapter configured: codelldb")
print("Adapter command: " .. dap.adapters.codelldb.executable.command)
print("Configurations available: " .. #dap.configurations.rust)

-- Set up basic keymaps for testing
vim.keymap.set('n', '<F5>', function() require('dap').continue() end, { desc = "DAP Continue" })
vim.keymap.set('n', '<F10>', function() require('dap').step_over() end, { desc = "DAP Step Over" })
vim.keymap.set('n', '<F11>', function() require('dap').step_into() end, { desc = "DAP Step Into" })
vim.keymap.set('n', '<F12>', function() require('dap').step_out() end, { desc = "DAP Step Out" })
vim.keymap.set('n', '<Leader>b', function() require('dap').toggle_breakpoint() end, { desc = "DAP Toggle Breakpoint" })
vim.keymap.set('n', '<Leader>dr', function() require('dap').repl.open() end, { desc = "DAP Open REPL" })
vim.keymap.set('n', '<Leader>dc', function() require('dap').run_to_cursor() end, { desc = "DAP Run to Cursor" })

print("\nKeymaps configured:")
print("  <F5> - Continue/Start debugging")
print("  <F10> - Step over")
print("  <F11> - Step into")
print("  <F12> - Step out")
print("  <Leader>b - Toggle breakpoint")
print("  <Leader>dr - Open REPL")
print("\nConfiguration loaded successfully!")
