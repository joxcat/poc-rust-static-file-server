use actix_files as fs;
use actix_web::http::header::{ContentDisposition, DispositionType};
use actix_web::{web, App, HttpRequest, middleware, HttpResponse, http::header, http::StatusCode, HttpServer, ResponseError};

use derive_more::{Display, Error};

use std::alloc::System;
#[global_allocator]
static A: System = System;

#[derive(Debug, Display, Error)]
enum ServerErrors {
	#[display(fmt = "Absolute path is forbidden")]
	AbsolutePath,
	#[display(fmt = "Relative access to parent is forbidden")]
	RelativeParentAccess,
	#[display(fmt = "404 not found")]
	CannotAccessFile { source: std::io::Error },
}

impl ResponseError for ServerErrors {
	fn status_code(&self) -> StatusCode {
		match self {
			ServerErrors::AbsolutePath => StatusCode::FORBIDDEN,
			ServerErrors::RelativeParentAccess => StatusCode::FORBIDDEN,
			ServerErrors::CannotAccessFile { source: _ } => StatusCode::NOT_FOUND
		}
	}

	fn error_response(&self) -> HttpResponse {
		actix_web::dev::HttpResponseBuilder::new(self.status_code())
			.set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
			.body(self.to_string())
	}
}

async fn index(req: HttpRequest) -> Result<fs::NamedFile, ServerErrors> {
	let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
	if path.starts_with("/") {
		return Err(ServerErrors::AbsolutePath);
	} else if path.starts_with(".") {
		return Err(ServerErrors::RelativeParentAccess);
	}

	let file = fs::NamedFile::open(path).map_err(|source| ServerErrors::CannotAccessFile {source})?;
	Ok(file
		.use_etag(true)
		.use_last_modified(true)
		.set_content_disposition(ContentDisposition {
			disposition: DispositionType::Attachment,
			parameters: vec![],
		}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let addr = std::env::args().collect::<Vec<String>>().get(1).cloned().unwrap_or_else(|| String::from("127.0.0.1:8080"));
	let cpus = num_cpus::get();

	println!("Server starting on {} and {} threads", addr, cpus);

	HttpServer::new(||
		App::new()
			.wrap(middleware::Compress::default())
			.service(web::resource("/{filename:.*}").to(index))
	)
	.bind(addr)?
	.workers(cpus)
	.run()
	.await
}
