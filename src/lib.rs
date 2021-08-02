// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub enum Tyfes {
    Nothing,
    Jpeg,
    Png,
    Gif,
    Bmp,
    WebP,
    Pdf
}

pub enum Markers {
    JpegSoi,
    JpegStart,

    PngSoi,
    PngStart2,
    PngStart3,
    PngStart4,

    GifSoi,
    GifStart2,
    GifStart3,

    BmpSoi,
    BmpStart2,

    WebpSoi,
    WebpStart2,
    WebpStart3,
    WebpStart4,

    PdfSoi,
    PdfStart2,
    PdfStart3,
    PdfStart4,

    IcoSoi,
    IcoStart3
}

pub struct Tyfe {
    extension: String,
    filename: String,
    binary_ext: Vec<String>,
}

impl Markers {
    fn val(&self) -> u8 {
        match *self {
            Markers::JpegSoi => 0xD8,
            Markers::JpegStart => 0xFF,

            Markers::PngSoi => 0x89,
            Markers::PngStart2 | Markers::WebpStart4 | Markers::PdfStart2 => 0x50,
            Markers::PngStart3 => 0x4E,
            Markers::PngStart4 | Markers::GifSoi => 0x47,

            Markers::GifStart2 => 0x49,
            Markers::GifStart3 | Markers::PdfStart4 => 0x46,

            Markers::BmpSoi | Markers::WebpStart3 => 0x42,
            Markers::BmpStart2 => 0x4D,

            Markers::WebpSoi => 0x57,
            Markers::WebpStart2 => 0x45,

            Markers::PdfSoi => 0x25,
            Markers::PdfStart3 => 0x44,

            _ => {
                0x00
            }
        }
    }
}

impl Default for Tyfe {
    fn default() -> Self {
        Tyfe {
            extension : "".to_string(),
            filename  : "".to_string(),
            binary_ext: vec![
                ".jpg",
                ".jpeg",
                ".png",
                ".gif",
                ".bmp",
                ".webp",
                ".pdf",
                ".ico"
            ].iter().map(|val| val.to_string()).collect(),
        }
    }
}

impl Tyfe {
    fn check(&mut self, file: String) -> Tyfes {
        self.filename = file;
        self.extension = std::path::Path::new(&self.filename)
            .extension()
            .and_then(std::ffi::OsStr::to_str).unwrap().to_string();

        return match &*self.extension.trim() {
            "jpg"  |
            "jpeg" |
            "png"  |
            "gif"  |
            "bmp"  |
            "webp" |
            "pdf" => {
                self.what_is_this()
            },
            _ => { Tyfes::Nothing }
        }
    }

    fn what_is_this(&self) -> Tyfes {
        let data = std::fs::read(&self.filename).unwrap();

        if *data.get(0).unwrap() == Markers::JpegStart.val()
            && *data.get(1).unwrap() == Markers::JpegSoi.val()
            && *data.get(2).unwrap() == Markers::JpegStart.val() {
            return Tyfes::Jpeg;
        }

        if *data.get(0).unwrap() == Markers::PngSoi.val()
            && *data.get(1).unwrap() == Markers::PngStart2.val()
            && *data.get(2).unwrap() == Markers::PngStart3.val()
            && *data.get(3).unwrap() == Markers::PngStart4.val() {
            return Tyfes::Png;
        }

        if *data.get(0).unwrap() == Markers::GifSoi.val()
            && *data.get(1).unwrap() == Markers::GifStart2.val()
            && *data.get(2).unwrap() == Markers::GifStart3.val() {
            return Tyfes::Gif;
        }

        if *data.get(0).unwrap() == Markers::BmpSoi.val()
            && *data.get(1).unwrap() == Markers::BmpStart2.val() {
            return Tyfes::Bmp;
        }

        if *data.get(8).unwrap() == Markers::WebpSoi.val()
            && *data.get(9).unwrap() == Markers::WebpStart2.val()
            && *data.get(10).unwrap() == Markers::WebpStart3.val()
            && *data.get(11).unwrap() == Markers::WebpStart4.val() {
            return Tyfes::WebP;
        }

        if *data.get(0).unwrap() == Markers::PdfSoi.val()
            && *data.get(1).unwrap() == Markers::PdfStart2.val()
            && *data.get(2).unwrap() == Markers::PdfStart3.val()
            && *data.get(3).unwrap() == Markers::PdfStart4.val() {
            return Tyfes::Pdf;
        }

        Tyfes::Nothing
    }
}


#[cfg(test)]
mod tests {
    use crate::{Tyfe, Tyfes};
    use std::io::{stdin};

    #[test]
    fn hmm() {
        let mut init = Tyfe::default();
        let file_exts = vec![
            ".jpg",
            ".gif",
            ".png",
            ".bmp",
            ".webp",
            ".pdf"
        ];

        for ext in file_exts {
            println!("{}", match init.check(format!("formats/test{}", ext).to_string()) {
                Tyfes::Jpeg => "JPEG",
                Tyfes::Png => "PNG",
                Tyfes::Gif => "GIF",
                Tyfes::Bmp => "BMP",
                Tyfes::WebP => "WebP",
                Tyfes::Pdf => "PDF",
                _ => "Hmm?"
            });
        }
    }
}
