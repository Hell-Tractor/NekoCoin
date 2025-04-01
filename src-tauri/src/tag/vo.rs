use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagVo {
    pub id: u32,
    pub name: String,
    pub remark: String,
    pub color: String,
    pub icon: String,
    pub parent_id: Option<u32>,
}