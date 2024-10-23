# Smart Hosts

Serve your smart hosts file as a DNS server.

> [!CAUTION]
> Working in progress.

## Usage

### Configuration

```plain
127.0.0.1   *.home.local, ssid="home"
127.0.0.2   *.home.local *.home.wg, ssid="work"
127.0.0.2   *.home.local *.home.wg, cellular="on"
```

## Development

### launch

```bash
cargo tauri dev
# Use below line to avoid rebuilding while developing with rust-analyzer
cargo tauri dev -- --target-dir ./target/dev 
```

### bundle

```bash
cargo tauri build --bundles app --target aarch64-apple-darwin
# cargo tauri build --bundles app --target x86_64-apple-darwin
# cargo tauri build --bundles app --target universal-apple-darwin
```
