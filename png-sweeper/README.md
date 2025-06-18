# PNG Sweeper

デスクトップにあるPNGファイル（スクリーンショット）を`~/Pictures/ScreenShot/`ディレクトリに自動的に移動するRustツールです。

## 機能

- デスクトップにあるすべてのPNGファイルを自動検出
- `~/Pictures/ScreenShot/`ディレクトリが存在しない場合は自動作成
- PNGファイルを安全に移動（ファイル名の重複チェック）
- 移動プロセスを日本語で表示

## インストール

```bash
# リポジトリをクローン
git clone <repository-url>
cd png-sweeper

# ビルド
cargo build --release
```

## 使用方法

### 開発環境での実行

```bash
cargo run
```

### リリースビルドの実行

```bash
./target/release/png-sweeper
```

## 動作例

```
$ cargo run
3個のPNGファイルを移動します...
移動しました: /Users/username/Desktop/Screenshot1.png -> /Users/username/Pictures/ScreenShot/Screenshot1.png
移動しました: /Users/username/Desktop/Screenshot2.png -> /Users/username/Pictures/ScreenShot/Screenshot2.png
移動しました: /Users/username/Desktop/Screenshot3.png -> /Users/username/Pictures/ScreenShot/Screenshot3.png
すべてのPNGファイルの移動が完了しました。
```

PNGファイルがない場合：
```
$ cargo run
デスクトップにPNGファイルが見つかりませんでした。
```

## 動作要件

- Rust 1.70.0以上
- macOS/Linux（HOME環境変数が必要）

## 注意事項

- このツールは**移動**操作を行います（コピーではありません）
- 移動先に同名ファイルがある場合は上書きされます
- 実行前に重要なファイルのバックアップを取ることをお勧めします

## ライセンス

MIT License
