# Image Upscaler

A command-line tool to upscale all JPEG images in a folder by 4x using high-quality Lanczos3 filtering. Optimized for Apple Silicon with support for parallel processing.

## Features

- Recursively finds all JPG/JPEG images in the specified folder
- Upscales images to 4x their original size using Lanczos3 algorithm
- Creates a `4x` subfolder for output images
- Processes images in parallel for better performance
- Supports custom thread count configuration
- Smart format detection (handles files even if extension doesn't match content)
- Native performance on Apple Silicon

## Installation

### Using Cargo

```bash
# Install directly from the repository
cargo install --path .

# Ensure ~/.cargo/bin is in your PATH (add to ~/.zprofile or ~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"
```

### Manual Installation

```bash
# Build the release binary
cargo build --release

# Copy to a system bin directory (requires sudo)
sudo cp target/release/img_upscaler /usr/local/bin/
```

## Usage

```bash
# Basic usage - process all images in a folder
img_upscaler /path/to/your/images

# Specify number of processing threads (defaults to CPU core count)
img_upscaler /path/to/your/images --threads 4

# Show help and all options
img_upscaler --help
```

### Examples

```bash
# Upscale all JPEGs in current folder
img_upscaler .

# Process images in Downloads with 8 threads
img_upscaler ~/Downloads --threads 8

# Upscale a specific album
img_upscaler ~/Pictures/vacation
```

## Output Structure

The tool processes your images as follows:
1. Scans input directory recursively for .jpg/.jpeg files
2. Creates a `4x` subfolder in the input directory
3. Saves upscaled copies with original filenames in the `4x` folder
4. Original files are never modified

For example:
```
input_folder/
  ├── photo1.jpg           # Original: 1000x1000
  ├── subfolder/
  │   └── photo2.jpg      # Original: 800x600
  └── 4x/                 # Created by the tool
      ├── photo1.jpg      # Upscaled: 4000x4000
      └── photo2.jpg      # Upscaled: 3200x2400
```

## Building from Source

```bash
# Clone the repository
git clone https://github.com/yehor-dykhov/img_upscaler.git
cd img_upscaler

# Build optimized release version
cargo build --release

# Run tests (optional)
cargo test

# Install globally (optional)
cargo install --path .
```

## Performance Tips

1. Always use `--release` builds for optimal performance
2. For bulk processing, experiment with `--threads` to find optimal count for your machine
3. Ensure sufficient disk space (output images are 16x larger in file size)
4. For Apple Silicon Macs, the binary is automatically optimized for ARM architecture

## Requirements

- Rust 2021 edition or later
- Sufficient disk space for output (approximately 16x the input size)
- Tested on macOS (Apple Silicon)

## License

MIT

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request