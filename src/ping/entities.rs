use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DBResponse {
    pub connection_exists: Option<i32>
}

#[derive(Serialize, Debug)]
pub struct PingResponse {
    pub status: String,
    pub data: DBResponse
}