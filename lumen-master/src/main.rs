#[derive(Debug, Clone)]
enum LightType {
    Dimmable{ brightness: u8 },
    OnOff{ is_on: bool},
}

#[derive(Debug, Clone)]
struct Light {
    id: u32,
    kind: LightType,
}

#[derive(Debug, Clone)]
struct Room {
    name: String,
    lights: Option<Vec<Light>>,
}

#[derive(Debug, Clone)]
enum ControlError {
    RoomEmpty,
    ConnectionErrorToRoom,
    LightFaulty
}

impl Room {
    fn calc_avg_brightness (&self) -> Result<(String, f64), (String, ControlError)> {
        match &self.lights {
            None            => Err((self.name.to_string(), ControlError::RoomEmpty)),
            Some(lights)    => {
                let mut avg_light: f64 = 0.0;
                for light in lights {
                    match light.kind {
                        LightType::Dimmable{ brightness } if brightness > 100 => return Err((self.name.to_string(), ControlError::LightFaulty)),
                        LightType::Dimmable{ brightness }           => avg_light += brightness as f64,
                        LightType::OnOff{ is_on } if is_on == true  => avg_light += 100.0,
                        LightType::OnOff{ .. }                      => avg_light += 0.0,
                    }
                }
                Ok((self.name.to_string(), avg_light / (lights.len() as f64)))
            }
        }
    }

}


fn main() {
    let house: Vec<Option<Room>> = vec![
        Some(Room{ 
            name: "LivingRoom".to_string(), 
            lights: Some(vec! [
                Light{ 
                    id: 1, 
                    kind: LightType::Dimmable{ 
                        brightness: 75 
                    }
                },
                Light{
                    id: 2,
                    kind: LightType::OnOff{
                        is_on: true
                    }
                },
                Light{
                    id: 3,
                    kind: LightType::OnOff{
                        is_on: false
                    }
                },
                ])
            }),
        None,
        Some(Room{ 
            name: "Kitchen".to_string(), 
            lights: { None },
            }),
        Some(Room{
            name: "MasterBedroom".to_string(),
            lights: Some(vec! [
                Light{
                    id:1,
                    kind: LightType::OnOff{
                        is_on: true
                    },
                }
            ])
        }),
        Some(Room{
            name: "Bedroom".to_string(),
            lights: Some(vec! [
                Light{
                    id:1,
                    kind: LightType::Dimmable{
                        brightness: 107,
                    },
                }
            ])
        })
    ];

    let report: Vec<Result<(String, f64), (String, ControlError)>> = house
        .iter()
        .filter_map(|rooms| match rooms {
            None        => Some(Err(("No Connection".to_string(), ControlError::ConnectionErrorToRoom))),
            Some(room)  => Some(room.calc_avg_brightness()),
        })
        .collect();

    dbg!(&report);
    
}
