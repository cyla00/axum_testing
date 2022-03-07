use axum::{
    response::Html,
};

pub async fn about() -> Html<&'static str>{
    Html("
    <div style='text-align: center;'>
        <h1>API</h1>
        <p>API root page</p>
    </div>
    ")
}