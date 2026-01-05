use rand::Rng;

#[derive(Debug, Clone, Copy)]
enum Room {
    LivingRoom,
    Kitchen,
    Bedroom,
    Cellar, 
    Children1,
    Children2,
}

#[derive(Debug, Clone, Copy)]
enum HeatingError {
    SensorOffline,
    RoomFrozen,
    TooHot,
}

fn check_temperature(_room: &Room, temp: &Option<f64>) -> Result<f64, HeatingError> {
    match temp {
        None                        => Err(HeatingError::SensorOffline),
        Some(temp) if *temp < 0.0    => Err(HeatingError::RoomFrozen),  
        Some(temp) if *temp > 50.0   => Err(HeatingError::TooHot),
        Some(temp)                  => Ok(*temp)
    }
}

fn main(){
    let measurements: Vec<(Room, Option<f64>)> = vec![
        (Room::LivingRoom, Some(23.5)),
        (Room::Kitchen, None),
        (Room::Bedroom, Some(56.0)),
        (Room::Children1, Some(rand::rng().random_range(-50.0..65.0))),
        (Room::Children2, Some(rand::rng().random_range(-50.0..65.0))),
        (Room::Cellar, None),
    ];

    println!("{:?}", measurements[0].0);

    for (room, temp) in &measurements {
        let result: Result<f64, HeatingError> = check_temperature(&room, &temp);
        match result {
            Ok(temp)                            => println!("It's quite comfortable in the {:?} with {:?} degrees.", room, temp),
            Err(HeatingError::RoomFrozen)       => println!("Attention! The {:?} is Frozen!", room),
            Err(HeatingError::TooHot)           => println!("Attention! The {:?} is overheating!", room),
            Err(HeatingError::SensorOffline)    => println!("Error! The sensor is offline. Please check connection."),
        };
    }


    let error_log: Vec<(Room, HeatingError)> = measurements
        .into_iter()
        .filter_map(|(room, temp)| {
            match check_temperature(&room, &temp) {
                Err(e) => Some((room, e)),
                Ok(_)  => None,
            }
        })
        .collect();

    println!("{:#?}", error_log);
}
