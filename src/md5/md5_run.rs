use crate::md5::md5_data::K;

fn round_1(w: u32, x: u32, y: u32, z: u32, m: u32, k: u32, s: u32) -> u32 {
    ((x & y) | (!x & z))
        .wrapping_add(w)
        .wrapping_add(m)
        .wrapping_add(k)
        .rotate_left(s)
        .wrapping_add(x)
}

fn round_2(w: u32, x: u32, y: u32, z: u32, m: u32, k: u32, s: u32) -> u32 {
    ((x & z) | (y & !z))
        .wrapping_add(w)
        .wrapping_add(m)
        .wrapping_add(k)
        .rotate_left(s)
        .wrapping_add(x)
}

fn round_3(w: u32, x: u32, y: u32, z: u32, m: u32, k: u32, s: u32) -> u32 {
    (x ^ y ^ z)
        .wrapping_add(w)
        .wrapping_add(m)
        .wrapping_add(k)
        .rotate_left(s)
        .wrapping_add(x)
}

fn round_4(w: u32, x: u32, y: u32, z: u32, m: u32, k: u32, s: u32) -> u32 {
    (y ^ (x | !z))
        .wrapping_add(w)
        .wrapping_add(m)
        .wrapping_add(k)
        .rotate_left(s)
        .wrapping_add(x)
}

pub fn run(message: &str) -> u128 {
    let mut buf: [u8; 64] = [0; 64];
    let msg_len = message.len() as u64 * 8;

    for (idx, byte) in message.bytes().enumerate() {
        buf[idx] |= byte; 
    }
    buf[message.len()] |= 0x80;

    for i in 0..8 {
        let current_len_byte = (msg_len >> (56 - (i * 8))) as u8; 
        buf[56 + i] |= current_len_byte; 
    }

    println!("BYTES:");
    for (idx, byte) in buf.iter().enumerate() {
        println!("{idx} :: {:b}", byte);
    }

    let mut state: [u32; 4] = [
        0x67452301,
        0xefcdab89,
        0x98badcfe,
        0x10325476,
    ];

    let mut m: [u32; 16] = [0; 16];
    for i in 0..16 {
        for j in 0..4 {
            m[i] = (m[i] << 8) | buf[(i * 4) + j] as u32;
        }
    }

    println!("M ARRAY:");
    for chunk in m {
        println!("{:b}", chunk);
    }

    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];

    // round 1 
    a = round_1(a, b, c, d, m[0], K[0], 7);
    d = round_1(d, a, b, c, m[1], K[1], 12);
    c = round_1(c, d, a, b, m[2], K[2], 17);
    b = round_1(b, c, d, a, m[3], K[3], 22);
    
    a = round_1(a, b, c, d, m[4], K[4], 7);
    d = round_1(d, a, b, c, m[5], K[5], 12);
    c = round_1(c, d, a, b, m[6], K[6], 17);
    b = round_1(b, c, d, a, m[7], K[7], 22);
    
    a = round_1(a, b, c, d, m[8], K[8], 7);
    d = round_1(d, a, b, c, m[9], K[9], 12);
    c = round_1(c, d, a, b, m[10], K[10], 17);
    b = round_1(b, c, d, a, m[11], K[11], 22);
    
    a = round_1(a, b, c, d, m[12], K[12], 7);
    d = round_1(d, a, b, c, m[13], K[13], 12);
    c = round_1(c, d, a, b, m[14], K[14], 17);
    b = round_1(b, c, d, a, m[15], K[15], 22);
    
    // round 2
    a = round_2(a, b, c, d, m[1], K[16], 5);
    d = round_2(d, a, b, c, m[6], K[17], 9);
    c = round_2(c, d, a, b, m[11], K[18], 14);
    b = round_2(b, c, d, a, m[0], K[19], 20);

    a = round_2(a, b, c, d, m[5], K[20], 5);
    d = round_2(d, a, b, c, m[10], K[21], 9);
    c = round_2(c, d, a, b, m[15], K[22], 14);
    b = round_2(b, c, d, a, m[4], K[23], 20);
    
    a = round_2(a, b, c, d, m[9], K[24], 5);
    d = round_2(d, a, b, c, m[14], K[25], 9);
    c = round_2(c, d, a, b, m[3], K[26], 14);
    b = round_2(b, c, d, a, m[8], K[27], 20);

    a = round_2(a, b, c, d, m[13], K[28], 5);
    d = round_2(d, a, b, c, m[2], K[29], 9);
    c = round_2(c, d, a, b, m[7], K[30], 14);
    b = round_2(b, c, d, a, m[12], K[31], 20);

    // round 3
    a = round_3(a, b, c, d, m[5], K[32], 4);
    d = round_3(d, a, b, c, m[8], K[33], 11);
    c = round_3(c, d, a, b, m[11], K[34], 16);
    b = round_3(b, c, d, a, m[14], K[35], 23);

    a = round_3(a, b, c, d, m[1], K[36], 4);
    d = round_3(d, a, b, c, m[4], K[37], 11);
    c = round_3(c, d, a, b, m[7], K[38], 16);
    b = round_3(b, c, d, a, m[10], K[39], 23);

    a = round_3(a, b, c, d, m[13], K[40], 4);
    d = round_3(d, a, b, c, m[0], K[41], 11);
    c = round_3(c, d, a, b, m[3], K[42], 16);
    b = round_3(b, c, d, a, m[6], K[43], 23);

    a = round_3(a, b, c, d, m[9], K[44], 4);
    d = round_3(d, a, b, c, m[12], K[45], 11);
    c = round_3(c, d, a, b, m[15], K[46], 16);
    b = round_3(b, c, d, a, m[2], K[47], 23);

    // round 4
    a = round_4(a, b, c, d, m[0], K[48], 6);
    d = round_4(d, a, b, c, m[7], K[49], 10);
    c = round_4(c, d, a, b, m[14], K[50], 15);
    b = round_4(b, c, d, a, m[5], K[51], 21);

    a = round_4(a, b, c, d, m[12], K[52], 6);
    d = round_4(d, a, b, c, m[3], K[53], 10);
    c = round_4(c, d, a, b, m[10], K[54], 15);
    b = round_4(b, c, d, a, m[1], K[55], 21);

    a = round_4(a, b, c, d, m[8], K[56], 6);
    d = round_4(d, a, b, c, m[15], K[57], 10);
    c = round_4(c, d, a, b, m[6], K[58], 15);
    b = round_4(b, c, d, a, m[13], K[59], 21);

    a = round_4(a, b, c, d, m[4], K[60], 6);
    d = round_4(d, a, b, c, m[11], K[61], 10);
    c = round_4(c, d, a, b, m[2], K[62], 15);
    b = round_4(b, c, d, a, m[9], K[63], 21);

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);

    let mut output: u128 = 0;
    for i in 0..4 {
        output = (output << 32) | state[i] as u128;
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::md5::md5_run::run;

    #[test]
    fn empty_string() {
        let text = "";
        let hash = run(text);

        println!("{:x}", hash);
        assert_eq!(hash, 0xd41d8cd98f00b204e9800998ecf8427e);
    }

    #[test]
    fn just_a() {
        let text = "a";
        let hash = run(text);

        println!("{:x}", hash);
        assert_eq!(hash, 0x0cc175b9c0f1b6a831c399e269772661);
    }

    #[test]
    fn abc() {
        let text = "abc";
        let hash = run(text);

        println!("{:x}", hash);
        assert_eq!(hash, 0x900150983cd24fb0d6963f7d28e17f72);
    }
}
