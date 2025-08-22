// タプルで定義されている。
/// 32bit整数定数
pub struct ConstInt(i32);

impl CosntInt {
    /// ConstIntを生成。
    pub fn new(value: i32) -> ConstInt {
        ConstInt(value);
    }

    /// getter。
    pub fn get(&self) -> i32 {
        self.0
    }
}
