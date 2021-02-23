use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig, MultipartOptions};
use async_graphql_actix_web::{Request, Response};
use async_graphql::{Context, EmptySubscription, Object, Schema, SimpleObject, Upload, ID};
use futures::lock::Mutex;
use slab::Slab;

pub type FilesSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone, SimpleObject)]
pub struct FileInfo {
    id: ID,
    filename: String,
    mimetype: Option<String>,
}

pub type Storage = Mutex<Slab<FileInfo>>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn uploads(&self, ctx: &Context<'_>) -> Vec<FileInfo> {
        let storage = ctx.data_unchecked::<Storage>().lock().await;
        storage.iter().map(|(_, file)| file).cloned().collect()
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn single_upload(&self, ctx: &Context<'_>, file: Upload) -> FileInfo {
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        println!("files count: {}", storage.len());
        let entry = storage.vacant_entry();
        let upload = file.value(ctx).unwrap();
        let info = FileInfo {
            id: entry.key().into(),
            filename: upload.filename.clone(),
            mimetype: upload.content_type,
        };
        entry.insert(info.clone());
        info
    }

    async fn multiple_upload(&self, ctx: &Context<'_>, files: Vec<Upload>) -> Vec<FileInfo> {
        let mut infos = Vec::new();
        let mut storage = ctx.data_unchecked::<Storage>().lock().await;
        for file in files {
            let entry = storage.vacant_entry();
            let upload = file.value(ctx).unwrap();
            let info = FileInfo {
                id: entry.key().into(),
                filename: upload.filename.clone(),
                mimetype: upload.content_type.clone(),
            };
            entry.insert(info.clone());
            infos.push(info)
        }
        infos
    }
}

async fn index(schema: web::Data<FilesSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(
                web::resource("/")
                    .guard(guard::Post())
                    .to(index)
                    .app_data(MultipartOptions::default().max_num_files(3)),
            )
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}