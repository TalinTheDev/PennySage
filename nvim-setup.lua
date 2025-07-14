-- Configures Neovim for this specific project

return {
	lsp = {
		rust_analyzer = {},
		cssls = {},
		html = {},
		eslint = {},
		jsonls = {},
		markdown_oxide = {},
		sqlls = {},
		taplo = {},
		volar = {},
		yamlls = {},
	},
	mason = {
		"fixjson",
		"prettierd",
	},
	treesitter = {
		"css",
		"csv",
		"dockerfile",
		"html",
		"javascript",
		"json5",
		"nginx",
		"rust",
		"sql",
		"toml",
		"vue",
		"yaml",
	},
}
