[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://gitpod.io/#FOLDER=tamagotchi/https://github.com/gear-foundation/dapps)
[![Docs](https://img.shields.io/github/actions/workflow/status/gear-foundation/dapps/contracts.yml?logo=rust&label=docs)](https://dapps.gear.rs/tamagotchi_io)

# Tamagotchi

### 🏗️ Building

```sh
cargo b -p tamagotchi -p "tamagotchi-[!b]*"
```

### ✅ Testing

```sh
cargo t -p tamagotchi -p "tamagotchi-[!b]*"
```

# 工作流程

首先要定义三个不可变更的函数，作为外部调用
```rs
#[no_mangle]
extern fn handle() {
    // 从接收到的消息中加载并解码成 TmgAction 类型。
    let action: TmgAction = msg::load().expect("Unable to decode `TmgAction`");
    // 获取或初始化 Tamagotchi 实例
    let tmg = unsafe { TAMAGOTCHI.get_or_insert(Default::default()) };
    match action {
        TmgAction::Name => {
            msg::reply(TmgReply::Name(tmg.name.clone()), 0)
                .expect("Error in a reply `TmgEvent::Name`");
        }
        TmgAction::Age => {
            let age = exec::block_timestamp() - tmg.date_of_birth;
            msg::reply(TmgReply::Age(age), 0).expect("Error in a reply `TmgEvent::Age`");
        }
        TmgAction::Feed => tmg.feed(),
        TmgAction::Play => tmg.play(),
        TmgAction::Sleep => tmg.sleep(),
        TmgAction::TmgInfo => tmg.tmg_info(),
    }
}
// 初始化函数
#[no_mangle]
extern fn init() {
    let TmgInit { name } = msg::load().expect("Failed to decode Tamagotchi name");
    let current_block = exec::block_timestamp();

    let tmg = Tamagotchi {
        name,
        date_of_birth: current_block,
        owner: msg::source(),
        fed: MAX_VALUE,
        fed_block: current_block,
        entertained: MAX_VALUE,
        entertained_block: current_block,
        rested: MAX_VALUE,
        rested_block: current_block,
    };
    unsafe {
        TAMAGOTCHI = Some(tmg);
    }
}
// 状态函数
#[no_mangle]
extern fn state() {
    let tmg = unsafe { TAMAGOTCHI.take().expect("Unexpected error in taking state") };
    msg::reply(tmg, 0).expect("Failed to share state");
}
```