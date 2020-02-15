use mime::Mime;
use std::ffi::OsStr;
use std::path::Path;

macro_rules! mime_table {
    ( $( $x:tt [ $y:tt $(, $z:tt)* ] ),* ) => {
            pub fn mime(path: &Path) -> &'static Mime {
                match path
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(str::to_lowercase)
                    .unwrap_or_else(String::new).as_str()
                {
                    $(
                        $y => &mime::$x,
                        $( $z => &mime::$x,)*
                    )*
                    _ => &mime::APPLICATION_OCTET_STREAM,
                }
            }

            pub fn extension(mime: &str) -> &'static str {
                $(
                    if mime == mime::$x {
                        concat!(".", $y)
                    } else
                )* {
                    ".xxx"
                }
            }
    };
}

mime_table!(
    APPLICATION_PDF ["pdf"],
    IMAGE_PNG ["png"],
    IMAGE_JPEG ["jpg", "jpeg"]
    );
