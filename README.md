# ColorLSP (fork)

> Fork of [huacnlee/color-lsp](https://github.com/huacnlee/color-lsp) with expanded language support, CSS4 color formats (oklch, oklab, lab, lch), and bug fixes.

[![Zed Extension](https://img.shields.io/badge/-Zed_Extension-blue?style=flat&logo=zedindustries&logoColor=%23FFFFFF&logoSize=auto&labelColor=%23111111&color=%23084CCF)](https://zed.dev/extensions/color-highlight)

ColorLSP is a Language Server Protocol (LSP) implementation for the highlight colors in documents, to provide [textDocument/documentColor](https://microsoft.github.io/language-server-protocol/specifications/specification-current/#textDocument_documentColor).

## Supported Color Formats

- **Hex**: `#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`
- **RGB / RGBA**: `rgb(255, 0, 0)`, `rgba(255, 0, 0, 0.5)`
- **HSL / HSLA**: `hsl(225, 100%, 70%)`, `hsla(20, 100%, 50%, 0.5)`
- **HWB**: `hwb(120 10% 20%)`
- **OKLCH**: `oklch(70% 0.15 180)`, `oklch(0.7 0.15 180 / 50%)`
- **OKLAB**: `oklab(0.7 0.1 -0.1)`
- **Lab / Lch**: `lab(50 30 -20)`, `lch(50 30 120)`
- **Rust hex literals**: `0xFF0000`, `0x00FF00AA`
- **GPUI format**: `rgb(0.5, 0.2, 0.1)` (Zed internal, values 0..1)

## Zed Color Highlight

<img width="1285" alt="SCR-20250626-oney" src="https://github.com/user-attachments/assets/a1a211d9-dec4-440b-8c74-848d7b03ff52" />

### Supported Languages

The extension activates on these languages by default:
HTML, CSS, JavaScript, TypeScript, JSX, TSX, Vue.js, Svelte, Astro, ERB, Tera, JSON, JSONC, JSON5, YAML, TOML, XML, Prisma, Rust, C, C++, Go, Zig, Python, Ruby, Dart, Java, Kotlin, Swift, PHP, Bash, Shell Script, Lua, Nix, Haskell, Elixir, Erlang, OCaml, Dockerfile.

### Enabling on Other Languages

The Zed extension API requires a predefined language list (see [upstream feature request](https://github.com/zed-industries/zed/discussions/45360)). To enable color-lsp on additional languages, add them to your Zed `settings.json`:

```jsonc
{
  "languages": {
    "Plain Text": {
      "language_servers": ["color-lsp"]
    },
    "Makefile": {
      "language_servers": ["color-lsp"]
    },
    // add any language here
  }
}
```

### Disabling Native Color Swatches

If you see double color swatches, disable Zed's built-in color rendering:

```jsonc
{
  "editor": {
    "lsp_document_colors": "none"
  }
}
```

## License

MIT
