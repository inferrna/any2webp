use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use image::codecs::webp::{WebPEncoder, WebPQuality};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::CreateKind;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");

    println!("watching {}", path);
    if let Err(e) = watch(path) {
        println!("error: {:?}", e)
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                //println!("changed: {:?}", event);
                proceed_event(event);
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn proceed_event(evt: Event) {
    if let EventKind::Create(CreateKind::File) = evt.kind {
        eprintln!("Fire create file evt!!");
        proceed_created_files(evt.paths)
    }
}

fn match_ext(p: &Path, extes: &[&str]) -> bool {
    for ext in extes {
        if p.extension() == Some(OsStr::new(ext)) {
            return true
        }
    }
    false
}

fn proceed_created_files(paths: Vec<PathBuf>) {
    for path in paths.into_iter() {
        let should_remove = if match_ext(&path, &["png", "bmp", "pnm", "tiff", "tif"]) {
            convert_file_to_webp(&path, WebPQuality::lossless());
            std::env::args().any(|c| c.eq("-rll")) //remove lossless
        } else if match_ext(&path, &["jpg", "jpeg"]) {
            convert_file_to_webp(&path, WebPQuality::lossy(92));
            std::env::args().any(|c| c.eq("-rls")) //remove lossy
        } else {
            false
        };
        if should_remove {
            fs::remove_file(&path).map_err(|e| {
                eprintln!("Can't remove old file {:?} because of error {e}", &path);
                e
            }).ok();
        }
    }
}

fn convert_file_to_webp(p: &PathBuf, q: WebPQuality) -> Option<()> {
    let img = image::open(p)
        .map_err(|e| {
            eprintln!("Can't open image {:?} because of error {e}", &p);
            e
        })
        .ok()?;
    let path_out = p.with_extension("webp");
    let file_writer = File::create(&path_out).map_err(|e| {
        eprintln!("Can't create image {:?} because of error {e}", &path_out);
        e
    }).ok()?;
    let writer = BufWriter::new(file_writer);
    let encoder = WebPEncoder::new_with_quality(writer, q);
    encoder.encode(img.as_bytes(), img.width(), img.height(), img.color())
        .map_err(|e| {
            eprintln!("Can't save image {:?} because of error {e}", &path_out);
            e
        }).ok()?;
    Some(())
}