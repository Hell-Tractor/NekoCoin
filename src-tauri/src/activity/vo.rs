use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateActivityVo {
    pub id: u32,
    pub name: String,
    pub remark: String,
    pub color: String,
    pub icon: String,
    pub open: bool,
    pub tag_id: u32,
}
