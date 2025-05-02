use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const DOCS_PATH: &str = "/docs";
const OPENAPI_PATH: &str = "/api-docs/openapi.json";

#[derive(OpenApi)]
pub struct Doc;

pub fn router() -> SwaggerUi {
    SwaggerUi::new(DOCS_PATH).url(OPENAPI_PATH, Doc::openapi())
}
