 生成全局UUID.扩展的雪花算法。可自定义存储u32数字，使用decode可解码出自定义数字

 *注意：使用大端存储*

 若返回int型`UUID::uuid(u32)`：会返回高位u64 和低位u32 俩个数字.
 高位存储时间戳和随机数，低位存储加密的自定义数字.
 > PHP版本 `new UUID()->uuid(int)`返回的是两个pack大端编码的字节数组

 返回String类型，会将高低位合并，然后使用 NO_PAD 和 URL安全的base64编码
 
 参考 雪花算法

 ## Example
 ------
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
 ```php
 $uuid = new UUID();
 for ($i = 0; $i < 100; $i++) {
     $encode = $uuid->uuid_string($i);
     $de = $uuid->decode($encode);
     echo $i, "-", $encode, "-", $de, "\n";
 }
 ```
