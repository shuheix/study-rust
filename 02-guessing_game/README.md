# 変数
* 可変にするためにはmutを付ける

- let fruit = "banana" //不変
- let mut fruit = "banana" // 可変(mutable)

# 参照

io::stdin()
        .read_line(&mut guess)

### &mut
- 引数が参照であることを示す
- コードの複数の部分が同じデータにアクセスしても、そのデータを何度もメモリにコピーしなくて済む機能

# コマンド
cargo doc --open
ローカルでdependenciesに追加したものをdocment形式でまとめてくれる
github見に行かなくてもいいみたいな感じ

