# Sequence Alignment Tool

Sequence Alignment Tool 是一款双序列/多序列比对工具，使用 Rust 语言编写。



## 实现原理

### CLUSTAL

CLUSTAL 算法由 Feng 和 Doolittle 等人于 1987 年提出，是一个渐进比对算法. 其通过重复利用双序列比对算法构建向导树，通过从最紧密的两条序列开始，不断引入临近（相似）的序列进行比对并不断重新构建比对，直到所有序列都被加入.

CLUSTAL 算法的时间复杂度较高，为 $O(n^4)$



## 使用方法

```bash
seqtool align --sequence-number 31 --fasta-path ./test.fasta --output-path ./result.sar
```

`--sequence-number, -n `，要比对的序列数量，当 `n=2` 时，使用 Needleman-Wunch 算法，当 `n>2` 时，使用 CLUSTAL 算法.

 ` --match-score, -m `，*可选*，配对成功的打分，默认值为 1.

 `--dismatch-score, -d`，*可选*，错配打分，一般为小于 0 的整数，默认值为 -2.

  ` --indel-score, -i`，*可选*，缺失罚分，一般为小于 0 的整数，默认值为 -5.

  `--fasta-path, -f`，输入的`.fasta`文件路径，该 fasta 文件应存放了所有待配对的序列，数目应与 `--sequence-number` 一致.

 ` --output-path, -o `，*可选*，输出的结果文件路径和名称，不填写时将自动输出至标准输出.



## 自行编译

您可以自行获取代码并进行编译.

### 获取代码

```bash
git clone https://github.com/bic-potato/SequenceAlignment
```

### Rust 环境获取

请参照 [Rust-lang 官方网站](https://www.rust-lang.org/tools/install).

### 编译

在源代码根目录下执行以下命令，编译完成后在 `target/release`文件夹下获取编译完成的二进制文件.

```bash
cargo build --release
```



