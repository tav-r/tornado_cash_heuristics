#[macro_export]
macro_rules! hashstring {
    ($l:expr) => {
        format!(
            "0x{}",
            $l.as_bytes()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        )
    };
}

pub mod analyze;
pub mod prepare;
