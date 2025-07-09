# VARYAG

VARYAG is a unified, fast, and intuitive network utility for developers, built in Rust. It aims to replace the fragmented command-line tools like `curl`, `httpie`, `netcat`, `websocat`, and `ngrok` by integrating their most common functionalities into a single, cohesive interface.

## Vision

The mission of VARYAG is to eliminate the cognitive overhead of switching between different network tools and their syntaxes. By providing a single, powerful, and easy-to-use CLI, VARYAG streamlines the daily workflows of backend developers, DevOps engineers, security specialists, and system administrators.

## Features

- **Unified Interface**: A single command structure for various network tasks.
- **Intelligent Protocol Detection**: Automatically detects the protocol (`http/s`, `ws/s`, `tcp`, `udp`) from the URL.
- **Rich Formatting**: Automatic JSON formatting and syntax highlighting for HTTP responses.
- **Comprehensive Commands**:
  - `varyag send`: Send network requests (HTTP, WebSocket, TCP, UDP).
  - `varyag listen`: Listen for incoming traffic, inspect requests, serve static files, or act as a proxy.
  - `varyag bridge`: Create public tunnels to your local services, replacing ngrok.

## Installation

*Coming soon...* (Instructions for installing via `cargo`, `brew`, `scoop`, and direct downloads will be added here).

## Usage

### `varyag send`

This command is your go-to tool for sending data across the network.

**Syntax:** `varyag send <DESTINATION> [METHOD] [BODY_ITEMS...] [OPTIONS]`

**Examples:**

- **Simple GET request:**
  ```bash
  varyag send https://api.example.com/users/1
  ```

- **POST request with JSON body:**
  ```bash
  varyag send api.example.com/users name="Varyag" status:='{"launched": 1899, "active": true}'
  ```

- **Set custom headers:**
  ```bash
  varyag send private.api/data -H "X-API-Key: my-secret-token" -H "Accept: application/json"
  ```

- **Send data from a file:**
  ```bash
  varyag send example.com/upload --data-file ./payload.json
  ```

- **Interact with a TCP service (e.g., Redis):**
  ```bash
  # One-shot command
  echo "GET my_key" | varyag send tcp://redis.local:6379

  # Interactive session
  varyag send tcp://redis.local:6379 -i
  ```

- **Send a WebSocket message:**
  ```bash
  varyag send ws://echo.websocket.events "Hello, WebSocket!"
  ```

### `varyag listen`

Inspect traffic, run simple servers, or proxy requests.

**Syntax:** `varyag listen <http|tcp|ws> [PORT] [OPTIONS]`

**Examples:**

- **Inspect incoming HTTP requests (great for webhooks):**
  ```bash
  varyag listen http 4000
  ```

- **Serve a local directory over HTTP:**
  ```bash
  varyag listen http 8080 --serve ./my-website
  ```

- **Proxy all incoming traffic to a local development server:**
  ```bash
  varyag listen http 3000 --proxy-pass http://localhost:8080
  ```

- **Run a TCP echo server:**
  ```bash
  varyag listen tcp 9000 --echo
  ```

### `varyag bridge`

Expose a local port to the internet.

**Syntax:** `varyag bridge <LOCAL_PORT> [OPTIONS]`

**Example:**

- **Create a public tunnel to your local port 3000:**
  ```bash
  varyag bridge 3000
  ```
  VARYAG will provide a public URL (e.g., `some-name.bore.pub`) that tunnels all traffic to `127.0.0.1:3000`.

## Development

This project is a Cargo workspace. To build it, you need the Rust toolchain installed.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/varyag.git
    cd varyag
    ```

2.  **Build the project:**
    ```bash
    cargo build --release
    ```
    The binary will be available at `target/release/varyag`.

## Contribution

Contributions are welcome! Please feel free to open an issue or submit a pull request.