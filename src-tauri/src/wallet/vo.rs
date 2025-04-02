use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateWalletVo {
    pub id: u32,
    pub name: String,
    pub remark: String,
    pub balance: u32,
    pub color: String,
    pub icon: String,
}