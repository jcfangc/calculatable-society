#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct DTOPropertyParams {
    pub frequency_constant: isize, // 频率常量 a
    pub phase_constant: isize,     // 相位常量 b
}
