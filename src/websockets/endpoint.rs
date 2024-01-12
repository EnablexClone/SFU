use actix::Addr;
use actix_web::{HttpRequest, web::{Payload, Path, Data}, Error, HttpResponse, get};
use actix_web_actors::ws;
use uuid::Uuid;

use super::{lobby::Lobby, connection::WsConn};

#[get("/{room_id}/{session_id}")]
pub async fn create_ws_connection(
    req: HttpRequest,
    stream: Payload,
    path: Path<(Uuid, Uuid)>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let query_params = path.into_inner();
    let session_id = query_params.1.clone();
    let room_id = query_params.0.clone();
    let ws = WsConn::new(
        srv.get_ref().clone(),
        session_id,
        room_id        

    );

    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}