use futures::{Stream, TryStreamExt};
use std::{
    io::Error as std_io_error,
    path::{Path, Component},
};
use axum::{BoxError, http::StatusCode, body::Bytes};
use tokio::{fs::File, io::BufWriter};
use tokio_util::io::StreamReader;

// Save a `Stream` to a file
pub async fn stream_to_file<S, E>(
    path: &str,
    stream: S,
) -> Result<(), (StatusCode, String)>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    if !path_is_valid(path) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid path".to_owned(),
        ));
    }

    async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(std_io_error::other);
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        let path = Path::new(super::constant::FILES_DIR).join(path);
        let mut file = BufWriter::new(File::create(path).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;

        Ok::<_, std_io_error>(())
    }
    .await
    .map_err(|err| {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    })
}

// to prevent directory traversal attacks
// ensure the path consists of exactly one normal component
fn path_is_valid(path: &str) -> bool {
    let path = Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}
