#![no_std]
use gstd::{debug, exec, msg, prelude::*};

#[no_mangle]
extern "C" fn handle() {
    // 获取合约消息负载
    // msg::load 函数从消息负载中读取数据，并返回一个 Result<T, E> 结果。
    // 如果消息负载无法解码为 T 类型，则返回一个 Err(E) 错误。
    let payload_string: String = msg::load().expect("Unable to decode `String`");
    // gstd::debug 宏可以在程序执行期间进行调试,debug! 宏只有在启用了 "debug" 特性时才可用：
    debug!("Received message: {payload_string:?}");
    // exec 模块可以获取当前执行上下文的信息
    // 获取当前区块的时间戳
    if exec::block_timestamp() >= 1672531200000 {
        //回复消息
        msg::reply(b"Current block has been generated after January 01, 2023", 0)
            .expect("Unable to reply");
    }
    // 获取消息发送者的账户ID
    let id = msg::source();
    let message_string = "Hello there".to_string();
    // 发送消息
    msg::send(id, message_string, 0).expect("Unable to send message");
}