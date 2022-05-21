# Hello, World
- タブではなく4スペースでインデントする
- 実行前にrustc main.rsコマンドでコンパイルする
- コンパイルに成功後、バイナリの実行可能ファイルが出力される(main)
- ./mainでmain.exeを実行させる

# Hello,Cargo
## cargo

### ビルド兼パッケージマネージャのこと
### コマンド
- cargo new hello_cargo
  - 雛形作成
- cargo build
  - ビルド
- ./target/debug/hello_cargo
  - ビルドで生成されたバイナリの実行
- cargo run
  - ビルドと実行を1stepで行う
- cargo check
  - バイナリを生成せずに、ビルドしてエラーがないか確認できる
  - よく使う
- cargo build --release
  - プロジェクトをリリース向けに最適化した状態でコンパイルできる


