# gemma-zed

Local Gemma 4 inference via LM Studio for Zed AI Assistant

## Overview

This Zed extension integrates [LM Studio](https://lmstudio.ai) with [Zed Editor](https://zed.dev), enabling you to use local Gemma 4 models as your AI coding assistant.

## Prerequisites

1. **Zed Editor** - Install from [zed.dev](https://zed.dev)
2. **LM Studio** - Install from [lmstudio.ai](https://lmstudio.ai)
3. **Rust** - Install from [rustup.rs](https://rustup.rs) (for building the bridge)
4. **Gemma 4 model** - Download in LM Studio (e.g., `gemma-4-e4b`)

## Installation

### Step 1: Build the Plugin

```bash
# Clone the repository
git clone https://github.com/irfancode/gemma-zed.git
cd gemma-zed

# Build the bridge binary
cd bridge
cargo build --release

# Build the extension (optional, for development)
cd ..
cargo build --release -p gemma-zed
```

### Step 2: Configure LM Studio

1. Open LM Studio
2. Download your preferred model (e.g., `gemma-4-e4b`)
3. Go to the **Developer** tab
4. Toggle **Start server** to enabled
5. Ensure the port is `1234` (or configure your preferred port)

### Step 3: Install the Extension in Zed

1. Open Zed Editor
2. Press `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Linux/Windows)
3. Click "Install Dev Extension"
4. Select the `gemma-zed` directory

### Step 4: Configure the Context Server

Add to your `settings.json`:

```json
{
  "context_servers": {
    "gemma-zed": {
      "command": "path/to/gemma-zed-bridge",
      "env": {
        "LM_STUDIO_PORT": "1234",
        "LM_MODEL": "gemma-4-e4b"
      }
    }
  }
}
```

Replace `path/to/gemma-zed-bridge` with the actual path to the compiled bridge binary.

## Usage

1. Ensure LM Studio server is running with your model loaded
2. In Zed, open the Agent Panel (`Cmd+Shift+I` / `Ctrl+Shift+I`)
3. Select or configure the Gemma context server
4. Start chatting with your local Gemma model!

## Architecture

```
┌─────────────────┐      MCP (stdio)       ┌─────────────────┐      HTTP      ┌─────────────────┐
│  Zed Agent      │ ◄────────────────────►│  Bridge Binary  │ ◄──────────────►│  LM Studio     │
│  (Agent Panel)  │                       │  (Rust binary) │                │  (HTTP API)    │
└─────────────────┘                       └─────────────────┘                └─────────────────┘
```

- **Zed Extension (WASM)**: Loads the bridge binary and configures settings
- **Bridge Binary**: Translates MCP JSON-RPC messages to LM Studio HTTP API calls

## Configuration Options

| Setting | Environment Variable | Default | Description |
|---------|------------------|---------|-------------|
| port | `LM_STUDIO_PORT` | `1234` | LM Studio API server port |
| model | `LM_MODEL` | `gemma-4-e4b` | Model identifier |

## Troubleshooting

### "Model not found" error
- Ensure the model is downloaded in LM Studio
- Verify the model name matches exactly (case-sensitive)

### "Connection refused" error
- Make sure LM Studio server is running
- Check the port matches in both LM Studio and settings

### Extension not loading
- Run Zed with `--foreground` flag to see debug output
- Verify the bridge binary path is correct in settings

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions welcome! Please open an issue or pull request on GitHub.