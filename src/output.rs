//! Utilities for outputting data.

use serde::{ser::SerializeSeq, Serialize};
use std::io::Write;

struct OutputCsvRow<'a> {
    name: &'a str,
    floats: &'a [f64],
    ints: &'a [u64],
}

impl Serialize for OutputCsvRow<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(5))?;

        seq.serialize_element(self.name)?;
        for val in self.floats {
            seq.serialize_element(val)?;
        }
        for val in self.ints {
            seq.serialize_element(val)?;
        }
        seq.end()
    }
}

/// Write the HW1 data for a specific network from FB100 dataset
/// We want the mean degree, mean neighbor degree, diameter,
/// and mean geodesic distance along with the name for labelling purposes
///
/// Writes to a csv specified by writer
/// 
/// Should work for both problems in HW1
pub fn to_csv<W: Write>(
    name: &str,
    floats: &[f64],
    ints: &[u64],
    writer: W,
) -> Result<(), csv::Error> {
    let output = OutputCsvRow {
        name,
        floats,
        ints,
    };

    let mut wtr = csv::Writer::from_writer(writer);

    wtr.serialize(output)
}
