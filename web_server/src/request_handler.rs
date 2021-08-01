use crate::http::request::Request;
use crate::http::response::Response;

pub struct RequestHandler {}

impl RequestHandler {
    pub fn new() -> Self {
        RequestHandler {}
    }

    pub fn handle(&self, request: &Request) -> Response {
        // Dani mete tu codigo aca, hace el match correspondiente y devolve una response
        // Para devolver la request hace Response::new(status_code, string)
        // Deje algo default para que compile
        Response::new().with_status(200).with_body(&format!(
            "Hola!!\nEsta fue tu request\n{}\n",
            request.to_string()
        ))
    }
}
