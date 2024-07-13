// https://gitlab.bixilon.de/bixilon/minosoft/-/raw/master/src/main/resources/assets/minosoft/mapping/versions.json
import versions from "./versions.json"

const versions_json: {
  pretty: string,
  safe: string,
  pvn: number,
}[] = []
const str: string[] = []

for (const v in versions) {
  const version = versions[v as keyof typeof versions]
  const name = version.name.replace(/[-\.]/g, "_").replace(/[<>]/g, "").toLocaleUpperCase()
  if (version['type' as keyof typeof version] === "release") {
    str.push(`/// Release ${version.name} (${v})`)
    str.push(`Release${name},`)

    versions_json.push({
      pretty: `Release ${version.name}`,
      safe: `Release${name}`,
      pvn: Number(v),
    })

    continue
  }
  const release_candidate = version.name.match(/-rc[0-9]+$/)
  if (release_candidate != null) {
    str.push(`/// Release Candidate ${version.name.replace(release_candidate[0], "")}/${release_candidate[0].replace("-rc", "")} (${v})`);
    str.push(`ReleaseCandidate${name},`)

    versions_json.push({
      pretty: `Release Candidate ${version.name.replace(release_candidate[0], "")}/${release_candidate[0].replace("-rc", "")}`,
      safe: `ReleaseCandidate${name}`,
      pvn: Number(v),
    })

    continue
  }
  const snapshot = version.name.match(/([0-9]+)w([0-9]+)([a-zA-Z]+)/)
  if (snapshot != null) { 
    str.push(`/// Snapshot ${snapshot[0]} (${v})`);
    str.push(`Snapshot${name},`)

    versions_json.push({
      pretty: `Snapshot ${snapshot[0]}`,
      safe: `Snapshot${name}`,
      pvn: Number(v),
    })

    continue
  }
  const pre_release = version.name.match(/(-pre([0-9]*))$/)
  if (pre_release != null) {
    str.push(`/// Pre Release ${version.name.replace(pre_release[0], "")}${pre_release[2] ? `/${pre_release[2]}` : ""} (${v})`);
    str.push(`PreRelease${name},`)

    versions_json.push({
      pretty: `Pre Release ${version.name.replace(pre_release[0], "")}${pre_release[2] ? `/${pre_release[2]}` : ""}`,
      safe: `PreRelease${name}`,
      pvn: Number(v),
    })

    continue
  }
  throw `Version ${v} has no protocol_id`
}

const file = `#![allow(non_camel_case_types)]

use crate::data_types::VarInt;

/// Enum of all versions of the game after the Netty rewrite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    ${str.join("\n    ")}
}

impl Version {
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn protocol_version_number(&self) -> VarInt {
        match self {
            ${versions_json.map(v => `Self::${v.safe} => VarInt::from(${v.pvn}),`).join("\n            ")}
        }
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn name(&self) -> &'static str {
        match self {
            ${versions_json.map(v => `Self::${v.safe} => "${v.pretty}",`).join("\n            ")}
        }
    }
}
`

Bun.write("../src/values/versions.rs", file)