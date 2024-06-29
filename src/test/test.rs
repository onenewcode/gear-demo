use gtest::{Program, System, Log};
use gstd::prelude::*;

#[test]
fn test_counter() {
    // 创建 System 实例
    let system = System::new();
    system.init_logger();

    // 上传和初始化智能合约
    // let program = Program::current(&system);
    let program = Program::from_file(
        &system,
        "./target/wasm32-unknown-unknown/release/counter.opt.wasm",
    );

    let result = program.send_bytes(0x02, "get");
    assert!(!result.main_failed());
    println!("0 get------->{:?}", result.log());

    // 发送 "inc" 消息
    // let result = program.send_bytes(0x02, b"inc");
    let result = program.send_bytes(0x02, b"inc");
    assert!(!result.main_failed());
    println!("1 inc------->{:?}", result.log());

    let result = program.send_bytes(0x02, "get");
    assert!(!result.main_failed());
    println!("1 get------->{:?}", result.log());
    let log = Log::builder()
        .source(program.id())
        .dest(0x02)
        .payload_bytes(b"1");
    assert!(result.contains(&log));

    // 发送 "dec" 消息
    let result = program.send_bytes(0x02, "dec");
    assert!(!result.main_failed());
    println!("0 dec------->{:?}", result.log());

    // 发送 "get" 消息并检查回复
    let result = program.send_bytes(0x02, "get");
    assert!(!result.main_failed());
    println!("0 get------->{:?}", result.log());

    let log = Log::builder()
        .source(program.id())
        .dest(0x02)
        .payload_bytes(b"0");
    assert!(result.contains(&log));
}