# Reproduction Steps

## 1. Clone this repository

```bash
git clone https://github.com/InkSha/read-image-but-return-zip
cd read-image-but-return-zip
```

## 2. Install dependencies

```bash
pnpm install
```

## 3. Start the upload server

```bash
pnpm run serve
```

## 4. Run the Tauri Android development build

 Open `src-tauri/src/lib.rs` and change the `LOCALHOST_IP_ADDRESS` constant to your local IP address:

```rust
// ...
// Change this line to your machine's local IP address on the network
const LOCALHOST_IP_ADDRESS: &'static str = "192.168.10.53";
// ...
```

```bash
pnpm run tauri android dev
```

The console will output content:

```bash
05-30 09:58:58.550 31786 31811 I RustStdoutStderr: path: asset://localhost/resources/images/icon.jpg
05-30 09:58:58.550 31786 31811 I RustStdoutStderr: mime: application/zip
05-30 09:58:58.550 31786 31811 I RustStdoutStderr: size: 393241626
05-30 09:58:58.550 31786 31811 I RustStdoutStderr: ready upload file "icon.jpg"
```

Check the uploaded file in the `project/uploads/` directory to confirm the upload succeeded.
