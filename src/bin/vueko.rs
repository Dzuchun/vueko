use ::Nuclide::Atom;
use clap::{Parser, ValueEnum};
use vueko::weizsacker;
use Nuclide::Nuclide;

#[derive(Debug, Clone, ValueEnum, derive_more::Display, Default)]
enum BEAlgorithm {
    /// Weizsacker 5-term formula
    #[display(fmt = "Weizsacker Formula")]
    #[default]
    Weizsacker,
    /// Nuclei built-in formula
    #[display(fmt = "Mass Model")]
    MassModel,
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = "A tiny tool to evaluate binding energy (BE) of known nucleons")]
struct Config {
    /// Algorithm to use for Binding Energy evaluation
    #[arg(default_value = "weizsacker")]
    algorithm: BEAlgorithm,
    /// Nucleon count of a nuclide (commonly referred as "A"). This also represents it's mass number and barion charge
    #[arg(short = 'A', long)]
    nucleons: Option<usize>,
    /// Proton count of a nuclide (commonly referred as "Z"). This also represents total electric charge of the nuclide
    #[arg(short = 'Z', long)]
    protons: Option<usize>,
    /// Neutron count of a nuclide (commonly referred as "N").
    #[arg(short = 'N', long)]
    neutrons: Option<usize>,
    /// Chemical identification of a nuclide. Must be a certain format, defined by Nuclide reference. For example: "He-3" or "U-235"
    #[arg(long = "chem")]
    chemical: Option<String>,
    /// A flag for raw result display. Intended for easier chaining or file output
    #[arg(short)]
    raw: bool,
    /// A flag to output BE energy per nucleon (total energy divided by A).
    #[arg(short)]
    per_nucleon: bool,
}

#[derive(Debug, ::thiserror::Error)]
enum Error {
    #[error("Failed to identify nuclide. Please specify at least two numbers or chemical element with nucleon count")]
    NotEnoughData,
    #[error("Failed to identify nuclide. Please only specify either \"A, Z, and/or N\", or chemical identifier, like \"U-235\"")]
    TooMuchData,
    #[error("Nuclide failed to create a nuclide. Seems like it does not exist")]
    NoNuclide,
    #[error("A must be at least as big as other count specified")]
    BadCounts,
}

fn main() -> Result<(), Error> {
    let config = Config::parse();
    let nuclide = match (
        config.neutrons,
        config.protons,
        config.nucleons,
        config.chemical,
    ) {
        // First, let's check if there's too much data
        (Some(_), _, _, Some(_)) | (_, Some(_), _, Some(_)) | (_, _, Some(_), Some(_)) => {
            return Err(Error::TooMuchData)
        }
        // Next, take care of literal nuclide representation
        (_, _, _, Some(chemical)) => Nuclide::new(chemical.as_str()).ok_or(Error::NoNuclide)?,
        // Only number'base identification left at this point:
        (n, z, a, _) => {
            let (n, z) = match (n, z, a) {
                // Not enough data case
                (None, None, None)
                | (None, None, Some(_))
                | (None, Some(_), None)
                | (Some(_), None, None) => return Err(Error::NotEnoughData),
                // Too much data case
                (Some(_), Some(_), Some(_)) => return Err(Error::TooMuchData),
                (None, Some(z), Some(a)) => {
                    if a >= z {
                        (a - z, z)
                    } else {
                        return Err(Error::BadCounts);
                    }
                }
                (Some(n), None, Some(a)) => {
                    if a >= n {
                        (n, a - n)
                    } else {
                        return Err(Error::BadCounts);
                    }
                }
                (Some(n), Some(z), None) => (n, z),
            };
            Nuclide::from_nucleons(z, n).ok_or(Error::NoNuclide)?
        }
    };

    let mut ans = match &config.algorithm {
        BEAlgorithm::Weizsacker => weizsacker::formula(&nuclide),
        BEAlgorithm::MassModel => nuclide.binding_energy(),
    };
    if config.per_nucleon {
        ans /= (nuclide.proton_neutron().0 + nuclide.proton_neutron().1) as f64;
    }

    if config.raw {
        println!("{ans}");
    } else {
        println!(
            "According to {}, binding energy{} of {} is {:.2} (MeV)",
            config.algorithm,
            if config.per_nucleon {
                " per nucleon"
            } else {
                ""
            },
            nuclide,
            ans
        );
    }
    Ok(())
}
