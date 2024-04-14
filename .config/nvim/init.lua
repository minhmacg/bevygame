require("minh")
--lazy
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({
    "git",
    "clone",
    "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git",
    "--branch=stable", -- latest stable release
    lazypath,
  })
end
vim.opt.rtp:prepend(lazypath)
require("lazy").setup({
    {
    "folke/which-key.nvim",
    event = "VeryLazy",
    init = function()
    vim.o.timeout = true
    vim.o.timeoutlen = 300
        end,
    opts = {
    -- your configuration comes here
    -- or leave it empty to use the default settings
    -- refer to the configuration section below
    }
    },
  	{ 'nvim-telescope/telescope.nvim', tag = '0.1.4',
    dependencies = { 'nvim-lua/plenary.nvim' }
    },
	{'/RRethy/vim-illuminate'},
	{'xiyaowong/transparent.nvim'},	
	{ 'nvim-treesitter/nvim-treesitter'},
	{ '/RRethy/nvim-base16' },
	{  'nvim-lualine/lualine.nvim',
    dependencies = { 'nvim-tree/nvim-web-devicons', opt = true }
	},
    { 'echasnovski/mini.pairs', version = false },
    { "lukas-reineke/indent-blankline.nvim", main = "ibl", opts = {} },
	{'williamboman/mason.nvim'},
	{'williamboman/mason-lspconfig.nvim'},
	{'VonHeikemen/lsp-zero.nvim', branch = 'v3.x'},
	{'neovim/nvim-lspconfig'},
	{'hrsh7th/cmp-nvim-lsp'},
	{'hrsh7th/nvim-cmp'},
	{'L3MON4D3/LuaSnip'},
})
--color
vim.cmd('colorscheme base16-horizon-terminal-dark')


require('mini.pairs').setup()
-- indent
require("ibl").setup()
--lualine
require('lualine').setup({
  options = { theme  = "base16" },
})
