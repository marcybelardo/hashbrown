use crate::md5::md5_data::{S, K};

pub fn run(message: &str) -> u128 {
    let mut buf: [u8; 64] = [0; 64];
    let msg_len = message.len() as u64 * 8;

    for (idx, byte) in message.bytes().enumerate() {
        buf[idx] |= byte; 
    }
    buf[message.len()] |= 0b10000000;

    for i in 0..8 {
        let current_len_byte = (msg_len >> (56 - (i * 8))) as u8; 
        buf[56 + i] |= current_len_byte; 
    }

    /*
    println!("BYTES:");
    for byte in buf {
        println!("{:08b}", byte);
    }
    */

    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;

    let mut m: [u32; 16] = [0; 16];
    for i in 0..16 {
        for j in 0..4 {
            m[i] <<= 8; 
            m[i] |= buf[(i * 4) + j] as u32;
        }
    }

    /*
    println!("M ARRAY:");
    for chunk in m {
        println!("{:032b}", chunk);
    }
    */

    let mut a = a0;
    let mut b = b0;
    let mut c = c0;
    let mut d = d0;

    for i in 0..64 {
        let mut f: u32;
        let g: usize;

        match i {
            0..=15 => {
                f = (b & c) | ((!b) & d);
                g = i;
            },
            16..=31 => {
                f = (d & b) | ((!d) & c);
                g = (5 * i + 1) % 16;
            },
            32..=47 => {
                f = b ^ c ^ d;
                g = (3 * i + 5) % 16;
            }, 
            _ => {
                f = c ^ (b | (!d));
                g = (7 * i) % 16;
            }
        }

        f = f.wrapping_add(a).wrapping_add(m[g]).wrapping_add(K[i]);
        a = d;
        d = c;
        c = b;
        b = b.wrapping_add(f.rotate_left(S[i]));
    }

    a0 = a0.wrapping_add(a);
    b0 = b0.wrapping_add(b);
    c0 = c0.wrapping_add(c);
    d0 = d0.wrapping_add(d);

    let mut output: u128 = 0;
    output |= a0 as u128;
    output <<= 32;  output |= b0 as u128;
    output <<= 32;  output |= c0 as u128;
    output <<= 32;  output |= d0 as u128;

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
