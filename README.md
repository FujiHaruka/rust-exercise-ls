# ls rust

- `ls` したらカレントディレクトリのファイル・ディレクトリ名をタブ文字区切りで標準出力に出す
  - `.` から始まるファイル名は出力しない
  - (optional) 標準出力以外では改行区切り
- `ls <directory>` ディレクトリを指定
- `ls <file>` ファイルが存在していればそれを表示し、存在しなければ `No such file or directory` で exit 1
- `-a` オプション - `.` で始まるファイル・ディレクトリ名も出力する
