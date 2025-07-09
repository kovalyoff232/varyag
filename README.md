# VARYAG

VARYAG is a modern, unified, and intuitive network utility for developers, designed to replace the fragmented command-line ecosystem of tools like `curl`, `httpie`, `netcat`, `websocat`, and `ngrok`.

The core mission of VARYAG is to reduce the cognitive load of switching between different tools and syntaxes by providing a single, powerful, and consistent interface for 95% of daily network-related tasks.

## Features

- **Unified Interface**: A single CLI for HTTP/S, WebSockets, TCP, and UDP.
- **Intuitive Syntax**: A command structure built around user intent (`send`, `listen`, `bridge`).
- **Smart Protocol Detection**: Automatically switches modes based on the URL scheme (`http://`, `ws://`, `tcp://`).
- **Beautiful Output**: Automatic JSON formatting and syntax highlighting.
- **Network Tunneling**: Expose local services to the internet with a single command, just like ngrok.
- **Cross-Platform**: Built in Rust, VARYAG is a single, fast, and reliable binary for all major platforms.

## Installation

Currently, you can install VARYAG from source using Cargo:
```bash
cargo install --git https://github.com/user/varyag.git # Replace with actual repo URL
```
Future versions will be available via Homebrew, Scoop, and as pre-compiled binaries in GitHub Releases.

## Usage

### `varyag send`

Replaces `curl` and `httpie`.

**Send a GET request:**
```bash
varyag send http GET https://api.example.com/users/1
```

**Send a POST request with data:**
```bash
varyag send http POST https://api.example.com/users name="Varyag" ship_class:='{"type": "Cruiser"}'
```

**Connect to a WebSocket and send a message:**
```bash
varyag send ws wss://echo.websocket.events "Hello, Varyag!"
```

**Send data over a raw TCP connection:**
```bash.
echo "GET_KEY my_key" | varyag send tcp 127.0.0.1:6379
```

### `varyag listen`

Replaces `netcat -l` and can serve static files.

**Inspect incoming HTTP requests (e.g., for webhooks):**
```bash
varyag listen http 4000
```

**Start a static file server in the current directory:**
```bash
varyag listen http 8000 --serve .
```

**Create a simple TCP echo server:**
```bash
varyag listen tcp 9000 --echo
```

### `varyag bridge`

Replaces `ngrok`.

**Expose a local web server on port 3000 to the internet:**
```bash
varyag bridge 3000
```
This will connect to a public server and provide you with a public URL that tunnels traffic to your local port 3000.

### Shell Completions

To generate shell completion scripts, use the `generate-completion` command.

**For Bash, add this to your `.bashrc`:**
```bash
eval "$(varyag generate-completion bash)"
```

**For Zsh, add this to your `.zshrc`:**
```bash
eval "$(varyag generate-completion zsh)"
```

**For Fish, add this to your `config.fish`:**
```bash
varyag generate-completion fish | source
```

## License

VARYAG is licensed under the MIT License.
