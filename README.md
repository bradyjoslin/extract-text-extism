# Extract Text Plugin

This project is a WebAssembly (WASM) plugin designed to extract text content from HTML using the `lol_html` library. The plugin is intended to be used with the Extism plugin development kit (PDK).

## Features
- Strips all HTML tags and returns plain text content.
- Uses `lol_html` for efficient and safe HTML parsing and rewriting.
- Can be compiled to WebAssembly (WASM) for cross-platform usage.

## Requirements

See [Extism Rust PDK](https://github.com/extism/rust-pdk) documentation.

## Usage

### Function: `extract`

The `extract` function is the main entry point of the plugin. It takes an HTML string as input and returns the text content after stripping all the HTML tags.

#### Parameters:
- `input: String` - The HTML content to be parsed.

#### Returns:
- `String` - The extracted plain text content.

### Example

Here is an example of how to use the `extract` function using [assembllm](https://github.com/bradyjoslin/assembllm/):

```yaml
name: "Extracted text summary"
description: |
  Using a prescript, get the contents of a remote URL, then use a wasm file to extract text from the content.  
  Finally, send the extracted text the LLM to summarize the content.

tasks:
  - name: summarize
    pre_script: |
      let content = Get("https://bradyjoslin.com/blog/remote-vs-code/");
      let wasm = "./target/wasm32-unknown-unknown/release/extract_text_extism.wasm";
      let res = Extism(wasm, "extract", content);
      res
    plugin: openai
    prompt: "summarize this article in 5 sentences"
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Extism](https://extism.io/) for the plugin development kit.
- [lol_html](https://github.com/cloudflare/lol-html) for the HTML rewriting library.
