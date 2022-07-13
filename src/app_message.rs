use super::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "tag", content = "content")]
pub(crate) enum AppMessage<'a> {
  Frame,
  Script(&'a str),
  Widget {
    key: &'a str,
    value: serde_json::Value,
  },
}
