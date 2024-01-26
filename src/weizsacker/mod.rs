use Nuclide::Nuclide;

const A1: f64 = 15.75;
const A2: f64 = 17.8;
const A3: f64 = 0.710;
const A4: f64 = 23.7;
const A5: f64 = 34f64;

pub fn formula(nuclei: &Nuclide) -> f64 {
    let (z, a) = nuclei.isotope();
    let delta = match (a % 2 == 0, z % 2 == 0) {
        (true, true) => {
            // even-even nuclei
            1.0
        }
        (true, false) | (false, true) => {
            // even-odd nuclei
            0.0
        }
        (false, false) => {
            // odd-odd nuclei
            -1.0
        }
    };
    let a: f64 = {
        let a: u32 = a.try_into().expect("Nucleon count is too big");
        a.into()
    };
    let z: f64 = {
        let z: u32 = z.try_into().expect("Proton count is too big");
        z.into()
    };
    A1 * a
        - A2 * a.powf(2.0 / 3.0)
        - A3 * z.powi(2) * a.powf(-1.0 / 3.0)
        - A4 * (a - 2.0 * z).powi(2) / a
        - A5 * delta / a.powf(3.0 / 4.0)
}
