# ヒューリスティックコンテスト ビジュアライザ開発のためのテンプレート
AHCのビジュアライザと似た機能を持ったビジュアライザを開発する際のテンプレートです。
コンテストごとに以下の処理を書き換えると、そのコンテスト用のビジュアライザをwebアプリ上で動かすことができます。
- seedが与えられたときに入力を生成する機能
- 入力と出力が与えられたときに、ビジュアライザの最大ターン数を計算する機能
- 入力・出力・ターン数が与えられたときに、スコアとそのときに表示するSVGを出力する機能

なお、これらの処理はRustで記述されていて、WebAssemblyによってフロントエンドのJavaScriptから呼び出されます。

また、フロントエンド部分はReactで記述されていますが、機能拡張が必要でなければ触れる必要はありません。

<img src="img/img1.png"/>

## 使用に際しての注意
使用については自己責任でよろしくお願いいたします。

また、コンテスト中の短期間で使うには何を編集するかをあらかじめ理解しておく必要があります。
必要に応じてビジュアライザを作る練習をすると良いと思います。

# 使用方法
このアプリを動かすには以下の環境が必要です:
- Rustの実行環境
- wasm-pack (https://developer.mozilla.org/ja/docs/WebAssembly/Rust_to_Wasm)
- nodeの実行環境
- yarn

これらを用意するためには
```
cargo install wasm-pack
npm install -g yarn
```
などを実行する必要があります。

初回実行時には以下のコマンドを実行してください:
```
yarn  # nodeのモジュールのインストール
cd wasm
wasm-pack build --target web --out-dir ../public/wasm  # Rustのwasm化
yarn dev # サーバーの実行
```
これでローカルにwebアプリがホスティングされるので、コンソール上に表示されるURLにアクセスしてください。テンプレートの状態のビジュアライザが表示されると思います。

# ビジュアライザ開発の手順
初期状態ではコンテストの問題に固有の情報が含まれていないので、URLにアクセスしてもページの雛形しか出てきません。
Rustのテンプレートを適切に編集をして、問題ごとのビジュアライザを作っていきます。

具体的にはRustの関数を3つ実装する必要があります(wasm/src/lib.rs):
- gen(seed: i32) -> String: seedを与えてStringの形で入力ファイルを出力する関数
- vis(_input: String, _output: String, turn: usize) -> Ret: 入力・出力・ターン数を与えて、その時点のスコア・エラー文・SVGの画像を返す関数
- get_max_turn(_input: String, _output: String) -> usize: 入力・出力を与えたときに、その出力が何ターンからなるものかを計算する関数(スライダーで動かすときなどに必要)

これらを適切に実装して、wasmのディレクトリに移動し
```
wasm-pack build --target web --out-dir ../public/wasm
```
Rustの関数をJavaScriptから呼び出せるようにwasm化するとビジュアライザが動くようになります。

具体的な実装は、yukicoder-score-contest002ブランチやchokduai-contest-005ブランチを参考にしてください。

# 開発tips

- 基本的にtodo!マクロの部分を実装すればよいです。コメントアウトすることで、テンプレートの初期実装で動作します。
- テンプレートの初期実装
  - input
    - n: 10～15の値: 緑の四角の横位置を決める
  - output
    - k: 灰色の丸の縦位置を決める。スペース区切りで出力し、値の数がturn数
- 図形の関数
  - create_react: 長方形
    - x, y: 左上の座標
    - width, height: 幅と高さ
    - fill: 長方形内の色(塗りつぶし), OptionでNoneだと黒
    - stroke: 枠線, OptionでNoneだと枠線なし
  - create_circle: 円
    - x, y: 中心の座標
    - r: 半径
    - fill, stroke: 長方形と同じ
  - create_line: 直線
    - x1, y1, x2, y2: 直線の端点
    - stroke_width: 線の幅
    - color: 線の色
  - create_text: テキスト
    - x, y: テキストの左上の座標(左上揃え)
    - size: フォントサイズ
    - s: テキストの内容
  - with_title: オブジェクトにタイトルを追加(マウスオーバーで表示)
    - node: 各createの関数の結果
    - title: 表示するタイトル
  - color: 色
    - RGB: RGBで色を指定
    - Name: 名前で色を指定(Fromトレイとを実装しており、intoで変換可能)
  - get_coler: 0.0～1.0で色を取得
    - 0.0が青で、0.5が緑、1.0が赤でグラデーションを作りやすい
  - stroke: 線のプロパティ
    - 0: 色
    - 1: 線の幅
