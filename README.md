# Rusty Tools

仕事や個人作業での面倒な作業を簡単に実行するためのRustツール集です。
このプロジェクトはRustのworkspace機能を利用して、複数の小さなツールを一元管理しています。

## 含まれるツール

### [png-sweeper](./png-sweeper/)
デスクトップにあるPNGファイル（スクリーンショット）を`~/Pictures/ScreenShot/`ディレクトリに自動的に移動するツールです。

### csv...
TODO:
- CSVファイルの値を使ってSQLクエリを分割作成する。
- 同じヘッダーのcsvファイルを結合する。


## セットアップ

```bash
# リポジトリをクローン
git clone <repository-url>
cd rusty-tools

# すべてのツールをビルド
cargo build --release
```

## 使用方法

### 特定のツールを実行

```bash
# png-sweeperを実行
cargo run -p png-sweeper
```

### リリースビルドの実行

```bash
# ビルド後の実行ファイルを直接実行
./target/release/png-sweeper
```

## 開発

新しいツールを追加する場合：

1. 新しいディレクトリを作成

```bash
cargo new --bin your-tools
```

2. `Cargo.toml`の`[workspace]`セクションの`members`配列に追加
3. 各ツールは独立したRustパッケージとして管理

