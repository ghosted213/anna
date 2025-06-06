// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use color_eyre::Result;
use winlock::Agent;

fn main() -> Result<()> {
    color_eyre::install()?;

    let _agent = Agent::new();

    Ok(())
}
