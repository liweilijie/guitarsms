# guitar

 将一个文件里面的所有通知新功能或者的描述也好，新的告警通知内容也好，拆分成一条一条短信发送出去。

 - 读取文件所有内容。
 - 拆分成70个字一个内容部分。
 - 分批次发送。
 - 发送需要有间隔几秒。

 ```bash
 ./target/debug/guitar -c m.md magic
 ./target/debug/guitar -c m.md me
 ./target/debug/guitar -c m.md trust
 ```
