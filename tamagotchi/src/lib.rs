#![no_std]

use codec::{Decode, Encode};
use gstd::{exec, msg, prelude::*, ActorId};
use tamagotchi_io::*;

#[derive(Default, Encode, Decode, TypeInfo)]
// 指定结构体编解码属性
#[codec(crate = gstd::codec)]
// 提供生成元数据功能
#[scale_info(crate = gstd::scale_info)]
// 设置我们宠物所包含的属性
struct Tamagotchi {
    // 姓名
    name: String,
    // 生日
    date_of_birth: u64,
//     // ActorId 表示智能合约参与者的标识。
    owner: ActorId,
    fed: u64,
    // 拟宠物最后一次被喂食的区块时间戳
    fed_block: u64,
    entertained: u64,
    entertained_block: u64,
    rested: u64,
    rested_block: u64,
}

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

impl Tamagotchi {
    // 投喂功能
    fn feed(&mut self) {
        assert!(!self.tmg_is_dead(), "Tamagotchi has died");
        // 更新饱食度
        self.fed += FILL_PER_FEED - self.calculate_hunger();
        // 取当前区块的时间戳
        self.fed_block = exec::block_timestamp();
        self.fed = if self.fed > MAX_VALUE {
            MAX_VALUE
        } else {
            self.fed
        };
        // 发送一个 TmgReply::Fed 消息，表示喂食成功。如果消息发送失败，则抛出错误。
        msg::reply(TmgReply::Fed, 0).expect("Error in a reply `TmgEvent::Fed`");
    }

    fn play(&mut self) {
        assert!(!self.tmg_is_dead(), "Tamagotchi has died");
        self.entertained += FILL_PER_ENTERTAINMENT - self.calculate_boredom();
        self.entertained_block = exec::block_timestamp();
        self.entertained = if self.entertained > MAX_VALUE {
            MAX_VALUE
        } else {
            self.entertained
        };
        msg::reply(TmgReply::Entertained, 0).expect("Error in a reply `TmgEvent::Entertained`");
    }

    fn sleep(&mut self) {
        assert!(!self.tmg_is_dead(), "Tamagotchi has died");
        self.rested += FILL_PER_SLEEP - self.calculate_energy();
        self.rested_block = exec::block_timestamp();
        self.rested = if self.rested > MAX_VALUE {
            MAX_VALUE
        } else {
            self.rested
        };
        msg::reply(TmgReply::Slept, 0).expect("Error in a reply `TmgEvent::Slept`");
    }
// 使用当前区块时间戳减去上次喂食的时间戳 self.fed_block，计算时间差并除以1000，然后乘以每区块增加的饥饿度 HUNGER_PER_BLOCK
    fn calculate_hunger(&self) -> u64 {
        HUNGER_PER_BLOCK * ((exec::block_timestamp() - self.fed_block) / 1_000)
    }

    fn calculate_boredom(&self) -> u64 {
        BOREDOM_PER_BLOCK * ((exec::block_timestamp() - self.entertained_block) / 1000)
    }

    fn calculate_energy(&self) -> u64 {
        ENERGY_PER_BLOCK * ((exec::block_timestamp() - self.rested_block) / 1000)
    }
    // 获取宠物信息
    fn tmg_info(&self) {
        msg::reply(
            TmgReply::TmgInfo {
                owner: self.owner,
                name: self.name.clone(),
                date_of_birth: self.date_of_birth,
            },
            0,
        )
        .expect("Error in a reply `TmgEvent::TmgInfo");
    }
    // 检查宠物是否死亡
    fn tmg_is_dead(&self) -> bool {
        let fed = self.fed.saturating_sub(self.calculate_hunger());
        let entertained = self.entertained.saturating_sub(self.calculate_boredom());
        let rested = self.rested.saturating_sub(self.calculate_energy());
        fed == 0 && entertained == 0 && rested == 0
    }
}

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

#[no_mangle]
extern fn state() {
    let tmg = unsafe { TAMAGOTCHI.take().expect("Unexpected error in taking state") };
    msg::reply(tmg, 0).expect("Failed to share state");
}
