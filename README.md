# Setup system

`scan_system`用の環境と動作確認をするためのプログラム

## Requirements

- Windows 10
- Anaconda3 or Miniconda3
- NI-VISA
- NI-488.2

## How to build 

```bash
cargo build --release
```

`target/release`内に生成された `setup_system.exe`を`scan_system`のプロジェクトルートに配置して使う。
