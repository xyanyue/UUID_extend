 生成全局UUID.扩展的雪花算法。可自定义存储u32数字，使用decode可解码出自定义数字

 *注意：使用大端存储*

> 作为UUID生成器，自定义数字可使用 机房+服务器等作为编码

> 作为ID转换器，自定义数字为数据库ID

 若返回int型 `UUID::uuid(u32)`：会返回高位u64 和低位u32 俩个数字.
 高位存储时间戳和随机数，低位存储加密的自定义数字.

> PHP版本 `new UUID()->uuid(int)`返回的是两个pack大端编码的字节数组

 返回String类型，会将高低位合并，然后使用 NO_PAD 和 URL安全的base64编码

 参考 雪花算法

## Rust Bench

> `Encode: encode      time:   [min:355.38 ns avg:358.24 ns max:361.84 ns]`
>
> `Decode: decode      time:   [min:214.15 ns avg:217.19 ns max:220.71 ns]`

## Example

---

#### Encode Decode String

```rust
     for i in 0..10 {
         let encode = UUID::uuid_string(i);
         let decode = UUID::decode(encode.clone());
         // println!("{}", encode,);
         assert_eq!(decode, i, "测试失败 with decode:{} and i:{}", decode, i);
     }
```

#### Encode Decode Int

```rust
 for i in 0..100 {
     let encode = UUID::uuid(i);
     let decode = UUID::decode_int(encode);
     // println!("{}", encode);
     assert_eq!(decode, i, "测试失败 with decode:{} and i:{}", decode, i);
 }
```

#### PHP 版本 。源码：php/lib.php

【https://github.com/xyanyue/UUID_extend】

```php
 $uuid = new UUID();
 for ($i = 0; $i < 100; $i++) {
     $encode = $uuid->uuid_string($i);
     $de = $uuid->decode($encode);
     echo $i, "-", $encode, "-", $de, "\n";
 }
```

## 样例

```
自定义数字：23424 - uuid:ZFyqoU5IAgUCBRXI - 解码：23424
自定义数字：23425 - uuid:ZFyqoQPtAgUCBVhs - 解码：23425
自定义数字：23426 - uuid:ZFyqoSmEAgUCBXIG - 解码：23426
自定义数字：23427 - uuid:ZFyqoaNaAgUCBfjZ - 解码：23427
自定义数字：23428 - uuid:ZFyqoSkXAgUCBXKT - 解码：23428
自定义数字：23429 - uuid:ZFyqoZE7AgUCBcq- - 解码：23429
自定义数字：23430 - uuid:ZFyqoQvBAgUCBVBH - 解码：23430
自定义数字：23431 - uuid:ZFyqoWegAgYCBjwn - 解码：23431
自定义数字：23432 - uuid:ZFyqobRPAgYCBu_H - 解码：23432
自定义数字：23433 - uuid:ZFyqofOBAgYCBqgI - 解码：23433
自定义数字：23434 - uuid:ZFyqof2IAgYCBqYC - 解码：23434
自定义数字：23435 - uuid:ZFyqoVEfAgYCBgqU - 解码：23435
```
