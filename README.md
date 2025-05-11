# nunix

> **Cross-platform Nix scripting powered by Nushell**

`nunix` is a lightweight extension for [Snix](https://snix.dev/) that brings the power of [Nushell](http://nushell.sh/) into the Nix ecosystem. With `nunix`, you can write and execute Nushell scripts directly inside your Nix expressions, enabling cross-platform scripting and data manipulation in pure Nix.

---

## 🚀 Features

* **Seamless Nushell Integration**: Leverage the expressive syntax and powerful data pipeline of Nushell within Nix.
* **Cross-Platform**: Works on Linux, macOS, and Windows without additional dependencies.
* **Structured Output**: Returns JSON-like Nix attribute sets for easy downstream processing.

---

## 🔧 Installation

1. Get it from github releases packages

2. nix shell github:rucadi/nunix

---

## 💡 Usage

The `builtins.nushell` function takes a single string parameter containing your Nushell script. It returns an attribute set with three fields:

```nix
{
  error     = { # Attribute set of any errors reported by Nushell };
  exit_code = Int;       # Exit code of the Nushell process
  output    = { # Result of evaluating the last pipeline as JSON
    <key> : <value>;     # Keys and values from the Nushell JSON output
    ...
  };
}
```

### Example

```nix
let
  result = builtins.nushell ''
    # List files in the current directory
     ls | where type == "file" | get name
  '';
in
  if result.exit_code != 0 then
    builtins.throw "Nushell error: ${result.error.message}"
  else
    result.output
```

This snippet runs a Nushell pipeline to list files in the current directory, filters for regular files, extracts their names, converts the list to JSON, and returns it as a Nix attribute set.

Running it from the current dir returns:
```json
[
  ".gitignore",
  "Cargo.lock",
  "Cargo.toml",
  "LICENSE",
  "README.md",
  "default.nix",
  "flake.lock",
  "flake.nix"
]
```

---

## 📚 API Reference

### `builtins.nushell(script: String) -> { error, exit_code, output }`

* **script**: A multiline string containing valid Nushell commands.
* **error**: An attribute set capturing any nushell runtime errors.
* **exit\_code**: Integer exit status of the Nushell shell (0 indicates success).
* **output**: Attribute set produced by `| to json` on the last pipeline.

---

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to:

* Open an issue in the [GitHub repository](https://github.com/rucadi/nunix).
* Submit pull requests with tests and documentation updates.


---

## 📜 License

This project is licensed under the MIT License. See [LICENSE](./LICENSE) for details.
