/*
 * vsclean - clear out pesky .vs folders
 * Copyright (C) 2024  Sebastian Pineda (spineda.wpi.alum@gmail.com)
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::error::Error;

use argparse::{ArgumentParser, Store, StoreTrue};

fn print_version_info() {
    const COPYRIGHT_NOTICE: &str =
        "Copyright (c) 2024 Sebastian Pineda (spineda.wpi.alum@gmail.com)
This program is free software; you may redistribute it under the terms of the
GNU General Public License version 3 or (at your option) any later version. This
program has absolutely no warranty.";
    const VERSION: &str = "0.0.1";
    const COOL_NAME_ART: &str = r"
                  __               
 _   ____________/ /__  ____ _____ 
| | / / ___/ ___/ / _ \/ __ `/ __ \
| |/ (__  ) /__/ /  __/ /_/ / / / /
|___/____/\___/_/\___/\__,_/_/ /_/ 
";

    println!("{}", COOL_NAME_ART);
    println!("Trace version {}\n", VERSION);
    println!("{}\n", COPYRIGHT_NOTICE);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut logging: bool = false;
    let mut print_version: bool = false;
    let mut directory: String = String::new();

    {
        let mut argument_parser: ArgumentParser = ArgumentParser::new();
        argument_parser.set_description("clear out pesky .vs folders");

        argument_parser.refer(&mut logging).add_option(
            &["-l", "--log"],
            StoreTrue,
            "Verbosely print",
        );

        argument_parser.refer(&mut print_version).add_option(
            &["-v", "--version"],
            StoreTrue,
            "Print version and license information",
        );

        argument_parser.refer(&mut directory).add_option(
            &["-d", "--directory"],
            Store,
            "Directory you would like to clear",
        );

        argument_parser.parse_args_or_exit();
    }

    if print_version {
        print_version_info();
        return Ok(());
    }

    Ok(())
}
