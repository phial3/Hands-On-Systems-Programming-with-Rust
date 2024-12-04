use clap::Parser;
use pipeviewer::{args::Args, read, stats, write};
use std::io::Result;
use std::thread;

fn main() -> Result<()> {
    let Args {infile, outfile, silent} = Args::parse();

    let (stats_tx, stats_rx) = crossbeam::channel::unbounded();
    let (write_tx, write_rx) = crossbeam::channel::bounded(1024);

    let read_handle = thread::spawn(move || {
        read::read_loop(
            &infile
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_default(),
            stats_tx,
            write_tx,
        )
    });
    let stats_handle = thread::spawn(move || stats::stats_loop(silent, stats_rx));
    let write_handle = thread::spawn(move || {
        write::write_loop(
            &outfile
                .as_ref()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_default(),
            write_rx,
        )
    });

    // crash if any threads have crashed
    // `.join()` returns a `thread::Result<io::Result<()>>`
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    // Return an error if any threads returned an error
    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}
