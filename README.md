# Vueko
This is a tiny single-function CLI tool using [Nuclide] to identify inputted nuclides and calculate their binding energy (BE).
Currently, BE can be computed using [Weizsacker formula] (with least-squares fit (2) coefficients) and [built-in Nuclide's formula] (seems to be Bethe-Weizsacker liquid-drop approximation, which might be the same but with different coefficients).
I plan on expanding functionality, if there would be more formula I'll need evaluation of during my nuclear physics zero-O-zero course.

Oh, and the name references character from [Made in Abyss], because I love random anime references.

## Releases
There is a single release built on my system (Arch Linux x86_64), so it might work for you, if you have similar one.

## Building
- Have [cargo installed]
- Download this repository
- Run:
```
cargo build release
```
- Find built binary at `./target/release/vueko`

## Usage
This tool uses [clap] to define it's CL args, so you can explore them by passing `--help` to it.
Here is a more detailed overview:

#### Algorithm (optional)
This argument is specified without any flag before it. Currently, there are two valid values:
- **weizsacker** for simple [Weizsacker formula] I've implemented myself
- **mass-model** for [built-in Nuclide's formula]

Defaults to weizsacker, if omitted.

#### -A, -Z and -N (--nucleons, --protons and --neutrons)
These define corresponding characteristics of a nuclide: total nucleon count, proton count and neutron count.
If `A` is less than either `Z` or `N`, tool exits with error.
If [Nuclide] fails to create nuclide with passed numbers, tool exits with error.

NOTE: exactly two of these must be defined. Program exits with error otherwise.
NOTE: all of these are exclusive with `--chem` argument, program exits with error otherwise.

#### --chem
Chemical nuclide identification can be passed with this argument. Internally, it relies on [Nuclide's `new` method].
Examples of valid inputs: "H-1", "Ne-20", "U-233".
Examples of invalid inputs: "92-233-U", "U-92-233", "triton", "alpha".

If input has invalid format, or [Nuclide] fails to identify specified nuclide, program exits with error.

NOTE: this argument is exclusive with `-A`, `-Z` and `-N`, program exits with error otherwise.

#### -r
A flag to output decimal number only (with no additional rounding applied). Intended to be used for command chaining.

#### -p
A flag to divide resulting energy by total nucleon count, resulting in binding energy per nucleon.


### Examples

```
> vueko -A 20 -N 10
According to Weizsacker Formula, binding energy of Ne-20 is 154.10 (MeV)
```

```
> vueko --chem He-3
According to Weizsacker Formula, binding energy of He-3 is 0.36 (MeV)
```

```
> vueko -Z 8 -N 9 -p
According to Weizsacker Formula, binding energy per nucleon of O-17 is 7.71 (MeV)
```

```
> vueko -Z 8 -N 9 -p mass-model
According to Mass Model, binding energy per nucleon of O-17 is 8.12 (MeV)
```

```
> vueko -Z 5 -A 10 -r -p
6.664089858202987
```




[Nuclide]: https://docs.rs/Nuclide/latest/Nuclide/
[Weizsacker formula]: https://en.wikipedia.org/wiki/Semi-empirical_mass_formula
[built-in Nuclide's formula]: https://docs.rs/Nuclide/latest/Nuclide/trait.Atom.html#tymethod.binding_energy
[clap]: https://docs.rs/clap/latest/clap/
[Nuclide's `new` method]: https://docs.rs/Nuclide/latest/Nuclide/struct.Nuclide.html#method.new
[cargo installed]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[Made in Abyss]: https://en.wikipedia.org/wiki/Made_in_Abyss