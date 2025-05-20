use clap::Parser;

/// OpenErase client software for securely sanitzing x86 devices
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args;
