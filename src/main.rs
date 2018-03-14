/*
 * Copyright (c) 2018 Pascal Bach
 *
 * SPDX-License-Identifier:     MIT
 */

use std::fs;
use std::path::Path;
use std::io;

extern crate rayon;

extern crate env_logger;
#[macro_use]
extern crate log;

#[macro_use]
extern crate clap;
use clap::{App, Arg};

fn main() {
    env_logger::init();

    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about("Walks a directory tree in parallel")
        .arg(
            Arg::with_name("ROOT")
                .help("Root to start walking the directory tree")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("read-content")
                .help("Read the content of each visited file")
                .short("r")
                .long("read-content"),
        )
        .get_matches();

    let path = matches
        .value_of("ROOT")
        .expect("ROOT should be set here. Something is wrong with clap.");
    debug!("Starting at: {}", path);

    let start = Path::new(&path);

    let read = matches.is_present("read-content");

    rayon::scope(|s| {
        visit_dirs(&start, s, read);
    });
}

fn read_file(path: &Path) -> std::io::Result<()> {
    let mut file = fs::File::open(path)?;
    let mut sink = io::sink();
    io::copy(&mut file, &mut sink)?;
    Ok(())
}

fn visit_dirs(dir: &Path, s: &rayon::Scope, read: bool) {
    if dir.is_dir() {
        match fs::read_dir(dir) {
            Ok(iter) => {
                iter.for_each(|entry| match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        info!("{:?}", path);
                        if path.is_dir() {
                            debug!("Entering: {:?}", path);
                            s.spawn(move |s| visit_dirs(&path, s, read));
                        } else {
                            if read {
                                debug!("Reading: {:?}", path);
                                s.spawn(move |_| {
                                    read_file(&path).unwrap_or_else(|e| {
                                        error!("Error reading file {:?}: {:?}", path, e)
                                    })
                                });
                            } else {
                                trace!("Skip reading: {:?}", path)
                            }
                        }
                    }
                    Err(e) => error!("Error iterating: {:?}", e),
                });
            }
            Err(e) => error!("Error entering directory {:?}: {:?}", dir, e),
        }
    }
}
