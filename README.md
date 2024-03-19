# 中国象棋学习工具

整体采用轻量级跨平台桌面应用方案`tauri`框架

前端: Vue3 + NaiveUI + TypeScript
后端: Rust + c + yolo + onnxruntime

# 线程

### 主线程

### 线程1

重复判断棋盘是否发生变化:

- 是 => 发送move事件到前端
  - 是否需要分析:
    - 是 => 发送fen到分析队列

### 线程2

监听分析队列，收到事件后:
调用云库查询, 是否查询成功 - 是 => 是否满足阈值设置: - 是 => 发送日志到前端 - 否 => 调用引擎 => 发送日志到前端 - 否 => 调用引擎 => 发送日志到前端

为这个函数编写一个单元测试(注意这里的`async runtime`用的是tokio)

中国象棋FEN的格式参考国际象棋基本原理，但有一些变化。

- 中国象棋棋盘大小固定为10行9列
- 中国象棋棋子使用单个字符表示，例如：k表示将，a表示士，b表示象，n表示马，r表示车，c表示炮，p表示兵。大写字符表示红方棋子，小写字符表示黑方棋子。
- 中国象棋棋盘默认黑方在上，红方在下方，FEN的的记录从上到下、从左排列
- 一个初始的棋盘FEN为：`rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w`, 其中最后的w代表此时该红方行棋。

现在请实现`pub async fn board_to_fen(camp: Camp, board: [[char; 9]; 10]) -> String {}`

帮我写实现parse函数, text的示例如下: score:0,depth:38,pv:h9g7|b0a2|h7i7|h0g2|i9h9|g3g4|c6c5|b2c2|b9c7|a0b0|c7d5|b0b4|b7d7|i0h0|h9h0|g2h0|d5e3|h0g2|d7e7|g2e3|e7e3|f0e1|c9e7|b4e4|e3a3|e2e6|d9e8|e6e5|a9d9|e4a4|a3i3|a4a6|i7h7|c2h2|d9d5|a6a9|d5d9|a9a6

```rust
pub struct QueryRecords {
    pub depth: usize,       // 深度
    pub score: usize,       // 得分
    pub time: usize,        // 时间
    pub pvs: Vec<String>, // 思考
}

pub fn parse(text: &str) -> QueryRecords {}
```

二进制命令文件`pikafish`
请帮我写一个UCI(Universal Chess Interface)引擎的控制器，使用struct封装起来，初始化传入一个uci引擎的文件路径， 并有一些方法实现调用逻辑,比如setoption、position、go等核心指令, 此外还需要注意两点：1、整体使用的是tokio async。2、go(depth time)输入指令和消息的输出应该使用队列可以异步进行

```
// 下面这里报错 mismatched types
// expected struct `tokio::sync::mpsc::Sender<UciCommand>`
//   found struct `tokio::sync::mpsc::Sender<UciResponse>`
UciEngineController {
    sender: tx,
    receiver: rx,
}
```
