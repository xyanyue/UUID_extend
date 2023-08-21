//! 生成全局UUID.扩展的雪花算法。可自定义存储u32数字，使用decode可解码出自定义数字
//!
//! *注意：使用大端存储*
//!
//! 若返回int型：会返回高位u64 和低位u32 俩个数字.
//! 高位存储时间戳和随机数，低位存储加密的自定义数字
//!
//! 返回String类型，会将高低位合并，然后使用 NO_PAD 和 URL安全的base64编码
//!
//! 参考 雪花算法
//!
//! ## Example
//! ------
//! #### Encode Decode String
//! ```no run
//!     for i in 0..10 {
//!         let encode = UUID::uuid_string(i);
//!         let decode = UUID::decode(encode.clone());
//!         // println!("{}", encode,);
//!         assert_eq!(decode, i, "测试失败 with decode:{} and i:{}", decode, i);
//!     }
//! ```
//! #### Encode Decode Int
//! ```no run
//! for i in 0..100 {
//!     let encode = UUID::uuid(i);
//!     let decode = UUID::decode_int(encode);
//!     // println!("{}", encode);
//!     assert_eq!(decode, i, "测试失败 with decode:{} and i:{}", decode, i);
//! }
//! ```
//! #### PHP 版本 。源码：php/lib.php
//! 【https://github.com/xyanyue/UUID_extend】
//! ```no run
//! $uuid = new UUID();
//! for ($i = 0; $i < 100; $i++) {
//!     $encode = $uuid->uuid_string($i);
//!     $de = $uuid->decode($encode);
//!     echo $i, "-", $encode, "-", $de, "\n";
//! }
//! ```
//!
use std::{
    ops::BitXor,
    // sync::atomic::{AtomicU32, Ordering},
    time::SystemTime,
};

use base64::{engine::general_purpose, Engine};
// static R: AtomicU32 = AtomicU32::new(1234);
// "qwerCXZLKJHtyuiopasdfghjklzxcvbnm0123456789MNBVGFDSAPOIUYTREWQ"
// const DEF: [u8; 62] = [
//     113, 119, 101, 114, 67, 88, 90, 76, 75, 74, 72, 116, 121, 117, 105, 111, 112, 97, 115, 100,
//     102, 103, 104, 106, 107, 108, 122, 120, 99, 118, 98, 110, 109, 48, 49, 50, 51, 52, 53, 54, 55,
//     56, 57, 77, 78, 66, 86, 71, 70, 68, 83, 65, 80, 79, 73, 85, 89, 84, 82, 69, 87, 81,
// ];
pub struct UUID {}

impl UUID {
    pub fn new() -> Self {
        Self {}
    }
    fn time_calculation() -> (u32, u16) {
        // 2023-05-10 00:00:00
        // 1683648000
        match SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => (
                (d.as_secs() & 0xffff_ffff) as u32,
                (d.subsec_millis() & 0x3ff) as u16,
            ),
            Err(_) => (0, 0),
        }
    }

    pub fn uuid(custom: u32) -> (u64, u32) {
        let rand = rand::random::<u16>();
        let (secs, mils) = UUID::time_calculation();

        let p1 = (secs as u64) << 32 | (rand as u64) << 16 | (mils as u64);

        let custom_xor = custom.bitxor(((mils as u32) << 16 | rand as u32) as u32);
        // println!("r:{} m:{} c:{}", rand, mils, custom_xor);
        (p1, custom_xor)
    }
    pub fn uuid_u128(custom: u32) -> u128 {
        let (p1, custom_xor) = UUID::uuid(custom);
        ((p1 as u128) << 32) | (custom_xor as u128)
    }
    pub fn uuid_string(custom: u32) -> String {
        let (p1, custom_xor) = UUID::uuid(custom);
        let mut byts = p1.to_be_bytes().to_vec();
        byts.append(&mut custom_xor.to_be_bytes().to_vec());
        // byts.
        // let number = UUID::uuid_u128(custom);
        general_purpose::URL_SAFE_NO_PAD.encode(byts)
    }

    pub fn decode(encode: String) -> u32 {
        let b = general_purpose::URL_SAFE_NO_PAD.decode(encode).unwrap();
        // println!("[u8]{:?}", b);
        let mils = ((b[6] as u16) << 8) | b[7] as u16;
        let rand = ((b[4] as u16) << 8) | b[5] as u16;

        let custom_xor =
            ((b[8] as u32) << 24) | ((b[9] as u32) << 16) | ((b[10] as u32) << 8) | b[11] as u32;
        // println!("rand:{} mils:{} custom_xor:{}", rand, mils, custom_xor);
        custom_xor.bitxor(((mils as u32) << 16 | rand as u32) as u32)
    }

    pub fn decode_int(encode: (u64, u32)) -> u32 {
        let rand = ((encode.0 & 0x00000000ffff0000) >> 16) as u16;
        let mils = (encode.0 & 0x000000000000ffff) as u16;

        encode.1.bitxor(((mils as u32) << 16 | rand as u32) as u32)
    }
    // 随机数生成算法
    // fn _random(&mut self) -> u16 {
    //     let seed = (75 * R.load(Ordering::Relaxed) + 74) % 2 ^ 16 + 1;
    //     R.store(seed, Ordering::Relaxed);
    //     seed as u16
    // }
}

#[test]
fn test() {
    for i in 23424..23524 {
        let encode = UUID::uuid_string(i);
        let cus = UUID::decode(encode.clone());
        println!("自定义数字：{} - uuid:{} - 解码：{}", i, encode, cus);
    }
}
// #[test]
// fn test_php() {
//     println!("{}", UUID::decode("ZFycveL0AAoACuKu".to_owned()));
// }

// #[test]
// fn test_1() {
//     let rand: u16 = 7533;
//     // let (secs, mils) = UUID::time_calculation();

//     let p1 = (1683788464 as u64) << 32 | (rand as u64) << 16 | (769 as u64);

//     let custom_xor = 100.bitxor(((769 as u32) << 16 | rand as u32) as u32);

//     println!("{:?}", p1.to_be_bytes());
// }
