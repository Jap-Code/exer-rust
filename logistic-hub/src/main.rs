#[derive(Debug)]
enum PackageType {
    Standard{ weight: f64 },
    Express{ weight: f64, priority: u32 },
    Fragile{ weight: f64, is_insured: bool },
}

#[derive(Debug)]
enum ShipmentError {
    Overweight,
    MissingInsurances,
    ScannerError
}

#[derive(Debug)]
struct Package {
    tracking_id: String,
    p_type: PackageType
}

impl Package {
    fn check_package(&self) -> Result<(), ShipmentError> {
        match &self.p_type {
            PackageType::Standard{ weight } | PackageType::Express{ weight, .. } | PackageType::Fragile{ weight, .. } if *weight >= 30.0 => Err(ShipmentError::Overweight),
            PackageType::Fragile{ is_insured, .. } if !is_insured => Err(ShipmentError::MissingInsurances),
            _ => Ok(()),
        }
    }
}

fn main() {
    let conveyor_belt: Vec<Option<Package>> = vec![
        Some(Package { 
            tracking_id: "001".to_string(), 
            p_type: PackageType::Standard{
                weight: 4.56
            }
        }),
        Some(Package {
            tracking_id: "002".to_string(),
            p_type: PackageType::Fragile{
                weight: 7.29,
                is_insured: false,
            }   
        }),
        None,
        Some(Package {
            tracking_id: "003".to_string(),
            p_type: PackageType::Express{
                weight: 31.45,
                priority: 1,
            }
        }),
        Some(Package {
            tracking_id: "004".to_string(),
            p_type: PackageType::Express{
                weight: 25.00,
                priority: 3,
            }
        }),
        None,
    ];

    let rejected_packages: Vec<(String, ShipmentError)> = conveyor_belt
        .iter()
        .filter_map(|pack| match pack {
            None            => Some(("xxx".to_string(), ShipmentError::ScannerError)),
            Some(package)   => {
                match package.check_package() {
                    Ok(())  => None,
                    Err(e)  => Some((package.tracking_id.to_string(), e)),
                }
            }
        })
        .collect();

        dbg!(&rejected_packages);
}