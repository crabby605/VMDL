# Vihaan's Minimal Data Language (VMDL)

A minimal, human-readable data structure language designed for configuration files and data exchange.
VMDL prioritizes readability and simplicity over complexity. It's designed to be instantly understandable by new and vibe coders while remaining easy to parse programmatically.

## Features
- **Minimal syntax**: No unnecessary brackets, quotes, or symbols
- **Comments**: Lines starting with `#` are comments
- **Key-value pairs**: Simple `Key = Value` format
- **Nested structures**: Indentation-based hierarchy
- **Multi-line values**: Natural text blocks

## Syntax Example

```vmdl
# Readable Format version of wrangler.toml
Project: my-worker
Type: javascript
Main: src/index.js
Workers Dev: false
Compatibility Date= 2025-06-05
Route= example.org/*
Zone Name= example.org
KV Namespaces:
    MY_NAMESPACE= <KV_ID>
Environments:
  Staging:
    Project Name= my-worker-staging
    Route= staging.example.org/*
    Zone Name= example.org
    KV Namespaces:
    MY_NAMESPACE= <STAGING_KV_ID>
Notes:
Configuration for a Cloudflare Worker with staging environment setup.
here worker uses KV storage and has custom routing for both production and staging environments.
```

## Language Support

- **Node.js**: `@vmdl/parser`
- **Rust**: `vmdl-rs`
- **Python**: `vmdl-python`
  
(this is still WIP. but the parser's should be published soon)
## Installation

### Node.js
```bash
# Using npm
npm install @vmdl/parser

# Using yarn
yarn add @vmdl/parser

# Using pnpm
pnpm add @vmdl/parser
```

### Rust
```toml
# Add to your Cargo.toml
[dependencies]
vmdl-rs = "0.0.1"
```

### Python
```bash
# Using pip
pip install vmdl-python

# Using poetry
poetry add vmdl-python
```

## Usage

### Node.js
```javascript
const fs = require('fs');
const { parse } = require('@vmdl/parser');

// Parse from file
const content = fs.readFileSync('config.vmdl', 'utf8');
const data = parse(content);

// Access parsed data
console.log(data.Project);         // "my-worker"
console.log(data.Route);           // "example.org/*"
console.log(data.Environments.Staging.Route); // "staging.example.org/*"

// Convert JavaScript object to VMDL
const { stringify } = require('@vmdl/parser');
const vmdlString = stringify(data);
fs.writeFileSync('new-config.vmdl', vmdlString);
```

### Rust
```rust
use std::fs;
use vmdl_rs::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse from file
    let content = fs::read_to_string("config.vmdl")?;
    let data = parse(&content)?;
    
    // Access parsed data
    println!("{}", data.get("Project").unwrap());  // "my-worker"
    println!("{}", data.get("Route").unwrap());    // "example.org/*"
    
    // Nested access
    let environments = data.get_object("Environments").unwrap();
    let staging = environments.get_object("Staging").unwrap();
    println!("{}", staging.get("Route").unwrap());  // "staging.example.org/*"
    
    Ok(())
}
```

### Python
```python
import vmdl_python as vmdl

# Parse from file
with open('config.vmdl', 'r') as f:
    data = vmdl.parse(f.read())

# Access parsed data
print(data["Project"])  # "my-worker"
print(data["Route"])    # "example.org/*"
print(data["Environments"]["Staging"]["Route"])  # "staging.example.org/*"
```

## Roadmap

- [x] Basic key-value parsing
- [x] Nested structures
- [x] Comments
- [ ] Arrays
- [ ] Boolean literals
- [ ] Numbers
- [ ] Strings with quotes
- [ ] Multi-line strings
- [ ] VSCode extension
- [ ] JetBrains plugin

## Contributing

I welcome contributions! This project follows semantic versioning and maintains backward compatibility.

Created by Vihaan P. - A readable approach to data configuration.