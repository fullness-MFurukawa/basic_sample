use serde::Deserialize;
//入力値を保持する構造体
#[derive(Deserialize , Debug)]
pub struct CalcForm{
    pub value1: i64, // 値1
    pub value2: i64, // 値2
    pub opt:    i32  // 計算の種類
}
