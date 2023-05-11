<?php
// 生成全局UUID.扩展的雪花算法。可自定义存储u32数字，使用decode可解码出自定义数字
//
// *注意：使用大端存储*
//
// 若返回int型：会返回高位64位byte 和低位32位 俩个pack.
// 高位存储时间戳和随机数，低位存储加密的自定义数字
//
// 返回String类型，会将高低位合并，然后使用 NO_PAD 和 URL安全的base64编码
class UUID
{
    function time_calculation(): array
    {
        $time = explode(' ', microtime());
        return [$time[1], intval($time[0] * 1000)];
    }
    function uuid(int $custom): array
    {
        $rand = rand(0, 65535);

        $time = $this->time_calculation();
        $secs = $time[0];
        $mils = $time[1];

        $p1 = pack("J", ($secs << 32 | $rand << 16 | $mils));
        $custom_xor = pack("N", $custom ^ ($mils << 16 | $rand));
        return [$p1, $custom_xor];
    }
    function base64url_encode($data)
    {
        return rtrim(strtr(base64_encode($data), '+/', '-_'), '=');
    }

    function base64url_decode($data)
    {
        return base64_decode(str_pad(strtr($data, '-_', '+/'), strlen($data) % 4, '=', STR_PAD_RIGHT));
    }
    function uuid_string(int $custom): string
    {
        $uuid = $this->uuid($custom);
        $bytes = $uuid[0] . $uuid[1];
        // $this->print_byte($bytes);
        return $this->base64url_encode($uuid[0] . $uuid[1]);
    }

    function decode(string $encode)
    {
        $b = $this->base64url_decode($encode);
        // $this->print_byte($b);
        $mils = (ord($b[6]) << 8) | ord($b[7]);
        $rand = (ord($b[4]) << 8) | ord($b[5]);

        $custom_xor =
            (ord($b[8]) << 24) | (ord($b[9]) << 16) | (ord($b[10]) << 8) | ord($b[11]);

        return $custom_xor ^ ($mils << 16 | $rand);
    }

    function print_byte($bytes)
    {
        echo "\n---------------\n";
        for ($i = 0; $i < strlen($bytes); ++$i) {
            echo ord($bytes[$i]), " ";
        }
        echo PHP_EOL;
        echo "\n---------------\n";
    }
}
