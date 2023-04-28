/// 生成全局UUID
/// 最大长度: 27个字符
/// 不定长度。组成部分为：(时间 + 自定义数字 + 线性同余方法（LCG）随机数) 然后 转62进制，码表自定义，没有使用通用码表
/// 码表 只包括数字、大小写字母
/// 如果码表不够 62 将用下划线代替
/// LCG 初始种子，默认选定的1234，所以这个随机数，是可预测的。我不用于加密，所以无所谓
/// 参考 雪花算法
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::SystemTime,
};
static R: AtomicU64 = AtomicU64::new(1234);
pub struct UUID<'a> {
    t: u64,
    n: u32,
    base62_str: &'a str,
}

impl<'a> UUID<'a> {
    pub fn new() -> Self {
        Self {
            t: 0,
            n: 0,
            base62_str: "qwerCXZLKJHtyuiopasdfghjklzxcvbnm0123456789MNBVGFDSAPOIUYTREWQ",
        }
    }
    // 假的日期计算.... 不想引入第三方库
    fn time_calculation(&mut self) -> String {
        let t = self.t / (86400);
        let y = t / 365;
        let m = (t - y * 365) / 30;
        let d = t - (y * 365) - (m * 30);

        let _d = self.t - (t * (86400));
        let h = _d / (3600);
        let i = (_d - (h * (3600))) / 60;
        let s = _d - (h * (3600)) - (i * 60);

        // self.u642u8(y);
        self.base62(y)
            + &self.base62(m)
            + &self.base62(d)
            + &self.base62(h)
            + &self.base62(i)
            + &self.base62(s)
            + &self.base62((self.n).into())
    }
    pub fn base62(&mut self, v: u64) -> String {
        let mut res = String::from("");
        let mut v = v;
        loop {
            let i = v % 62;

            match self.base62_str.as_bytes().get(i as usize) {
                Some(_v) => res.push_str(_v),
                None => res.push_str("_"),
            }
            if v < 62 {
                break;
            }
            v = v / 62;
        }
        return res;
    }
    // fn u642u8(&self, v: u64) -> u8 {
    //     if v > 255 {
    //         return 255;
    //     }
    //     v as u8
    // }

    pub fn next(&mut self, custom: u32) -> String {
        match SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(d) => {
                self.t = d.as_secs();
                self.n = d.subsec_nanos();
            }
            Err(_) => {}
        }

        let seed = self.random();

        //// 计算
        // self.time_calculation();
        // let mut source = self.base62(custom.into());
        // source = source + &self.base62(seed);
        // self.source = self.source.to_owned() + &source;
        // self.source.to_owned()
        format!(
            "{}_{}_{}",
            self.time_calculation(),
            self.base62(custom.into()).clone(),
            self.base62(seed)
        )
    }
    fn random(&mut self) -> u64 {
        let seed = (8121 * R.load(Ordering::Relaxed) + 28411) % 134456;
        R.store(seed, Ordering::Relaxed);
        seed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut uuid = UUID::new();
        println!("{:?}", uuid.next(4_294_967_295));
    }
}
