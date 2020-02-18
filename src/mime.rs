use mime::Mime;
use std::ffi::OsStr;
use std::path::Path;

pub trait MimeExt {
    fn mime(&self) -> &'static Mime;
}

pub trait ExtensionExt {
    fn extension(&self) -> &'static str;
}

macro_rules! mime_table {
    ( $( $x:tt [ $y:tt $(, $z:tt)* ] ),* ) => {
            fn mime(path: &Path) -> &'static Mime {
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

            fn extension(mime: &str) -> &'static str {
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

impl MimeExt for Path {
    fn mime(&self) -> &'static mime::Mime {
        mime(self)
    }
}

impl ExtensionExt for Mime {
    fn extension(&self) -> &'static str {
        extension(self.as_ref())
    }
}
