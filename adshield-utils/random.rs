use rand::rngs::{OsRng, StdRng, ThreadRng};
use rand::Rng;

impl RandExt for OsRng {}
impl RandExt for ThreadRng {}
impl RandExt for StdRng {}

pub trait RandExt: Rng {
    fn gen_class_name(&mut self) -> String {
        const CHARSET0: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                _";
        const CHARSET1: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789_";

        let mut result = String::with_capacity(16);
        result.push(CHARSET0[self.gen_range(0..CHARSET0.len())] as char);
        for _ in 9..19 {
            result.push(CHARSET1[self.gen_range(0..CHARSET1.len())] as char)
        }

        result
    }

    fn gen_id_name(&mut self) -> String {
        const CHARSET0: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                _";
        const CHARSET1: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789_";

        let mut result = String::with_capacity(16);
        result.push(CHARSET0[self.gen_range(0..CHARSET0.len())] as char);
        for _ in 9..19 {
            result.push(CHARSET1[self.gen_range(0..CHARSET1.len())] as char)
        }

        result
    }

    fn gen_ext(&mut self) -> &str {
        static POOL: &[&str] = &[
            "avif", "pdf", "ps", "class", "pict", "webp", "eps", "pls", "csv", "mid", "doc",
            "midi", "ppt", "tif", "xls", "docx", "jar", "pptx", "tiff", "xlsx",
        ];

        POOL[self.gen_range(0..POOL.len())]
    }

    fn gen_path(&mut self) -> String {
        const CHARSET0: &[u8] = b"ABCEFGHIJKLMNOPQRSTUVWXYZ\
                                abcefghijklmnopqrstuvwxyz\
                                _";

        const CHARSET1: &[u8] = b"ABCEFGHIJKLMNOPQRSTUVWXYZ\
                                abcefghijklmnopqrstuvwxyz\
                                _";

        let mut result = String::with_capacity(32);
        result.push(CHARSET1[self.gen_range(0..CHARSET1.len())] as char);

        let range0: usize = self.gen_range(8..28);
        for i in 0..range0 {
            if let Some(last_char) = result.as_bytes().last() {
                if last_char == &b'/' {
                    result.push(CHARSET1[self.gen_range(0..CHARSET1.len())] as char);
                }
            }

            if i != range0 {
                if self.gen_range(0..5) == 0 {
                    result.push('/');
                }

                if self.gen_range(0..5) == 1 {
                    result.push('.');
                }
            }
            result.push(CHARSET0[self.gen_range(0..CHARSET0.len())] as char);
        }
        result.push(CHARSET1[self.gen_range(0..CHARSET1.len())] as char);
        result
    }
}
