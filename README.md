# snb

[![Crates.io](https://img.shields.io/crates/v/snb)](https://crates.io/crates/snb)

Superfast, minimalist note-taking from your terminal [^1]

A lightning-fast CLI note-taking tool written in Rust, inspired by [nb](https://github.com/xwmx/nb). Perfect for developers who want to quickly capture thoughts without leaving their terminal.

## Features

- 🚀 Blazingly fast performance
- 📝 Simple markdown-based note creation and editing
- 🔍 Quick note lookup and listing
- ⌨️ Terminal-first workflow
- 📂 Minimal configuration required

## Installation

### Via Cargo

```bash
cargo install snb
```

### Dependencies

- Rust 1.70 or higher
- A text editor (set via `EDITOR` environment variable)

## Usage

### Notes Management

```bash
# Create a new note
snb add "This is a note" --title "My Awesome note"

# List all notes
snb

# Edit note by ID
snb edit <id>

# Delete note by ID
snb delete <id>
```

Note: Deleted note IDs are preserved (holes in sequence) for consistency with nb behavior.

### Configuration

For now, set the `EDITOR` environment variable manually.


## Roadmap

- [ ] Folder management
- [ ] Bookmark support
- [ ] Todo list functionality
- [ ] Index rebuilding
- [ ] Note search and filtering
- [ ] Tags support
- [ ] Configuration file support

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -am 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please ensure your PR:
- Follows the existing code style
- Includes tests if applicable
- Updates documentation as needed

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

[^1]: Name suggested by https://claude.ai/ 🙏
