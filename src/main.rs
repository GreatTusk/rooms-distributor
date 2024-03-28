use room::Room;

use crate::housekeeper::Housekeeper;
use crate::room::RoomType;

mod room;
mod housekeeper;

fn main() {
    let rooms = create_rooms_matrix();
    let housekeepers =
        [Housekeeper::new(housekeeper::Shift::PartTime, "Bob".to_string(), '1'),
            Housekeeper::new(housekeeper::Shift::PartTime, "Dave".to_string(), '1'),
            Housekeeper::new(housekeeper::Shift::FullTime, "Alice".to_string(), '2'),
            Housekeeper::new(housekeeper::Shift::FullTime, "Charlie".to_string(), '3'),
            Housekeeper::new(housekeeper::Shift::FullTime, "Eve".to_string(), '4')];
    let work_unit_quota = distribute_work_unit_quota(&rooms, &housekeepers);
    // const ROOM_TYPES: usize = RoomType::COUNT;

    let result = arrange_rooms(rooms, housekeepers, work_unit_quota);
    println!("{:?}", result);

}

fn arrange_rooms(mut rooms: Vec<Room>, housekeepers: [Housekeeper; 5], mut work_unit_quota: Vec<u8>) -> Vec<Vec<Room>> {
    // The amount of housekeepers determines the sizes of the arrays
    let housekeeper_count: usize = housekeepers.len();

    // One vector per housekeeper to store the rooms they will clean
    let mut room_arrangement = get_room_arrangement(housekeeper_count);
    let mut room_quantity = get_room_count_by_type(&rooms);
    let mut bed_pool = work_unit_quota.iter().sum::<u8>();
    let room_work_units: [u8; 3] = [RoomType::Double.get_work_units(), RoomType::Triple.get_work_units(), RoomType::Quad.get_work_units()];
    bed_pool = assign_single_room(&mut rooms, &mut bed_pool, &housekeepers, &mut room_arrangement, &mut work_unit_quota);
    bed_pool = assign_equally_if_less_than_six(&room_quantity, &mut bed_pool, housekeeper_count as u8, &mut rooms, &mut room_arrangement);

    while bed_pool > 0 {

        if let Some(new_value) = is_last_workload(&work_unit_quota, &mut room_arrangement, &mut bed_pool, &mut rooms) {
            bed_pool = new_value;
        } else if let Some(new_value) = can_any_quota_be_satisfied(&mut work_unit_quota,
                                                                   &mut room_quantity, &room_work_units, &mut bed_pool, &mut rooms,
                                                                   &mut room_arrangement, &housekeepers) {
            bed_pool = new_value;
        } else if let Some(new_value) = can_make_work_quota_even(&mut work_unit_quota,
                                                                 &mut room_quantity, &room_work_units, &mut bed_pool, &mut rooms,
                                                                 &mut room_arrangement, &housekeepers) {
            bed_pool = new_value;
        } else if let Some(new_value) = should_assign_two_rooms(&mut work_unit_quota,
                                                                &mut room_quantity, &room_work_units, &mut bed_pool, &mut rooms,
                                                                &mut room_arrangement, &housekeepers) {
            bed_pool = new_value;
        } else if let Some(new_value) = assign_evenly(&mut work_unit_quota,
                                                      &mut room_quantity, &room_work_units, &mut bed_pool, &mut rooms,
                                                      &mut room_arrangement, &housekeepers) {
            bed_pool = new_value;
        }
    }

    room_arrangement
}

fn assign_evenly(work_unit_quota: &mut Vec<u8>, room_quantity: &mut [usize; 3], room_work_units: &[u8; 3],
                 bed_pool: &mut u8, rooms: &mut Vec<Room>,
                 room_arrangement: &mut Vec<Vec<Room>>, housekeepers: &[Housekeeper]) -> Option<u8> {
    let index_of_double = RoomType::get_index(&RoomType::Double) - 1;
    for i in 0..work_unit_quota.len() {
        if work_unit_quota[i] <= 0 {
            continue;
        }

        if room_quantity[index_of_double] > 0 {
            assign_room(work_unit_quota, room_quantity, room_work_units, bed_pool, rooms, room_arrangement, index_of_double, i, 1, housekeepers)?;
        }
    }
    return Some(*bed_pool);
}

fn assign_room(work_unit_quota: &mut Vec<u8>, room_quantity: &mut [usize; 3], room_work_units: &[u8; 3],
               bed_pool: &mut u8, rooms: &mut Vec<Room>, room_arrangement: &mut Vec<Vec<Room>>,
               room_index: usize, housekeeper_index: usize, rooms_to_subtract: usize, housekeepers: &[Housekeeper]) -> Option<()> {
    room_arrangement[housekeeper_index].push(get_room_of_type(rooms, RoomType::get_value(room_index + 1).unwrap(), housekeeper_index, housekeepers)?);
    room_quantity[room_index] -= rooms_to_subtract;
    work_unit_quota[housekeeper_index] -= room_work_units[room_index];
    *bed_pool -= room_work_units[room_index];
    Some(())
}

fn should_assign_two_rooms(work_unit_quota: &mut Vec<u8>, room_quantity: &mut [usize; 3], room_work_units: &[u8; 3],
                           bed_pool: &mut u8, rooms: &mut Vec<Room>,
                           room_arrangement: &mut Vec<Vec<Room>>, housekeepers: &[Housekeeper; 5]) -> Option<u8> {

    // Before continuing, consider that threes can be problematic if the workload is not divisible by 3
    // Thus, two have to be assigned as soon as possible to avoid problems. Before invoking this functions, uneven numbers should have been dealt with, so it should be safe to do so

    let triple_room_index = RoomType::get_index(&RoomType::Triple) - 1;

    let index_of_six = work_unit_quota.iter().position(|value| *value == 6);

    if index_of_six.is_some() && room_quantity[triple_room_index] > 2 {
        let quota_of_six = index_of_six.unwrap();
        room_arrangement[quota_of_six].push(get_room_of_type(rooms, RoomType::Triple, quota_of_six, housekeepers)?);
        room_arrangement[quota_of_six].push(get_room_of_type(rooms, RoomType::Triple, quota_of_six, housekeepers)?);
        room_quantity[triple_room_index] -= 2;
        work_unit_quota[quota_of_six] -= room_work_units[triple_room_index] * 2;
        *bed_pool -= room_work_units[triple_room_index] * 2;
        return Some(*bed_pool);
    }


    None
}

fn can_make_work_quota_even(work_unit_quota: &mut Vec<u8>, room_quantity: &mut [usize; 3], room_work_units: &[u8; 3],
                            bed_pool: &mut u8, rooms: &mut Vec<Room>,
                            room_arrangement: &mut Vec<Vec<Room>>, housekeepers: &[Housekeeper]) -> Option<u8> {
    let triple_room_index = RoomType::get_index(&RoomType::Triple) - 1;
    if room_quantity[triple_room_index] <= 0 {
        return None;
    }
    let index_of_fixable_workload = work_unit_quota.iter().position(|value| value % 2 != 0)?;
    assign_room(work_unit_quota, room_quantity, room_work_units, bed_pool, rooms, room_arrangement, triple_room_index, index_of_fixable_workload, 1, housekeepers)?;
    return Some(*bed_pool);
}

fn can_any_quota_be_satisfied(work_unit_quota: &mut Vec<u8>, room_quantity: &mut [usize; 3], room_work_units: &[u8; 3],
                              bed_pool: &mut u8, rooms: &mut Vec<Room>,
                              room_arrangement: &mut Vec<Vec<Room>>, housekeepers: &[Housekeeper]) -> Option<u8> {
    for i in 0..work_unit_quota.len() {
        if work_unit_quota[i] <= 0 {
            continue;
        }
        for j in 0..room_quantity.len() {
            if room_quantity[j] > 0 && work_unit_quota[i] - room_work_units[j] == 0 {
                // Will never panic because the condition above guarantees that the room is present
                assign_room(work_unit_quota, room_quantity, room_work_units, bed_pool, rooms, room_arrangement, j, i, 1, housekeepers)?;
                return Some(*bed_pool);
            }
        }
    }
    None
}

fn is_last_workload(work_unit_quota: &Vec<u8>, room_arrangement: &mut Vec<Vec<Room>>, bed_pool: &mut u8, rooms: &mut Vec<Room>) -> Option<u8> {
    return if work_unit_quota.iter().filter(|value| **value != 0).count() == 1 {
        let last_housekeeper_index = work_unit_quota.iter().position(|&value| value != 0).unwrap();
        room_arrangement[last_housekeeper_index].extend(rooms.drain(..));
        *bed_pool = 0;
        Some(*bed_pool)
    } else {
        None
    };
}

fn assign_equally_if_less_than_six(room_quantity: &[usize; 3],
                                   bed_pool: &mut u8, housekeeper_count: u8, rooms: &mut Vec<Room>,
                                   room_arrangement: &mut Vec<Vec<Room>>) -> u8 {
    let total_rooms = room_quantity.iter().sum::<usize>();
    if total_rooms < 6 {
        while !rooms.is_empty() {
            for i in 0..housekeeper_count {
                if rooms.is_empty() {
                    break;
                }
                room_arrangement[i as usize].push(rooms.swap_remove(0));
            }
        }
        0
    } else {
        *bed_pool
    }
}


fn get_room_of_type(rooms: &mut Vec<Room>, room_type: RoomType, housekeeper_index: usize, housekeepers: &[Housekeeper]) -> Option<Room> {
    let preferred_floor = housekeepers[housekeeper_index].preferred_floor;
    // Getting the index of the first room of the specified type
    let room_index = rooms
        .iter()
        .position(|room| room.room_type == room_type && room.id.to_string().get(0..1).unwrap() == preferred_floor.to_string());
    match room_index {
        Some(index) => Some(rooms.swap_remove(index)),
        None => {
            // If no room of the specified type is found on the preferred floor, get any room of the specified type
            let room_index = rooms
                .iter()
                .position(|room| room.room_type == room_type);
            match room_index {
                Some(index) => Some(rooms.swap_remove(index)),
                None => None,
            }
        }
    }
}

fn get_single_room(rooms: &mut Vec<Room>) -> Option<Room> {
    let single_room = rooms.iter().position(|room| room.room_type == RoomType::Single)?;
    Some(rooms.swap_remove(single_room))
}

fn assign_single_room(rooms: &mut Vec<Room>, bed_pool: &mut u8, housekeepers: &[Housekeeper], room_arrangement: &mut Vec<Vec<Room>>, work_unit_quota: &mut Vec<u8>) -> u8 {
    match get_single_room(rooms) {
        Some(single_room) => {
            // Operations to assign the room to the last housekeeper (who will have the largest workload)
            let last_index = housekeepers.len() - 1;
            room_arrangement[last_index].push(single_room);
            work_unit_quota[last_index] -= 1;
            *bed_pool -= 1;
            *bed_pool
        }
        None => {
            *bed_pool
        }
    }
}

fn get_room_arrangement(housekeeper_count: usize) -> Vec<Vec<Room>> {
    let mut room_arrangement: Vec<Vec<Room>> = Vec::with_capacity(housekeeper_count);

    for _ in 0..housekeeper_count {
        room_arrangement.push(Vec::new());
    }
    room_arrangement
}

fn get_room_count_by_type(rooms: &[Room]) -> [usize; 3] {
    let room_counts = [RoomType::Double, RoomType::Triple, RoomType::Quad]
        .iter()
        .map(|room_type| rooms.iter().filter(|room| room.room_type == *room_type).count())
        .collect::<Vec<_>>();
    [room_counts[0], room_counts[1], room_counts[2]]
}

fn get_total_work_units(rooms: &[Room]) -> u8 {
    rooms.iter().map(|room| room.room_type.get_work_units()).sum()
}

fn distribute_work_unit_quota(rooms: &[Room], housekeepers: &[Housekeeper]) -> Vec<u8> {
    let total_work_units = get_total_work_units(rooms);
    let (full_time, part_time) = get_shift_distribution(housekeepers);
    let work_ratio = 1.6;

    let work_unit_divider = (part_time as f64 + work_ratio * full_time as f64).round();
    let equal_parts = (total_work_units as f64 / work_unit_divider).round();

    // The vector is initialized with equal parts for each housekeeper
    let mut work_unit_distribution = vec![equal_parts; housekeepers.len()];

    for i in part_time..housekeepers.len() {
        work_unit_distribution[i] = (work_unit_distribution[i] * work_ratio).round();
    }

    let work_unit_distribution = work_unit_distribution.iter().map(|value| *value as u8).collect::<Vec<_>>();

    adjust_work_unit_distribution(work_unit_distribution, total_work_units)
}

fn adjust_work_unit_distribution(mut work_unit_distribution: Vec<u8>, total_work_units: u8) -> Vec<u8> {
    let current_work_units = work_unit_distribution.iter().sum::<u8>();

    if current_work_units == total_work_units { return work_unit_distribution.clone(); }

    return if current_work_units > total_work_units {
        reduce_work_units(&mut work_unit_distribution, total_work_units, current_work_units)
    } else {
        increase_work_units(&mut work_unit_distribution, total_work_units, current_work_units)
    };
}

fn reduce_work_units(work_unit_distribution: &mut Vec<u8>, total_work_units: u8, mut current_work_units: u8) -> Vec<u8> {
    loop {
        for unit in work_unit_distribution.iter_mut().rev() {
            *unit -= 1;
            current_work_units -= 1;
            if current_work_units == total_work_units { return work_unit_distribution.clone(); }
        }
    }
}

fn increase_work_units(work_unit_distribution: &mut Vec<u8>, total_work_units: u8, mut current_work_units: u8) -> Vec<u8> {
    loop {
        for unit in work_unit_distribution.iter_mut() {
            *unit += 1;
            current_work_units += 1;
            if current_work_units == total_work_units { return work_unit_distribution.clone(); }
        }
    }
}

fn get_shift_distribution(housekeepers: &[Housekeeper]) -> (usize, usize) {
    let full_time = housekeepers
        .iter()
        .filter(|h| h.shift == housekeeper::Shift::FullTime)
        .count();
    let part_time = housekeepers.len() - full_time;
    (full_time, part_time)
}

fn create_rooms_matrix() -> Vec<Room> {
    let mut rooms_matrix: Vec<Room> = Vec::new();

    // First floor
    rooms_matrix.push(Room::new(101, RoomType::Triple));
    rooms_matrix.push(Room::new(102, RoomType::Double));
    rooms_matrix.push(Room::new(105, RoomType::Double));
    rooms_matrix.push(Room::new(104, RoomType::Single));

    // Second floor
    for i in 206..=216 {
        rooms_matrix.push(Room::new(i, RoomType::Double));
    }

    // Third floor
    rooms_matrix.push(Room::new(317, RoomType::Triple));
    rooms_matrix.push(Room::new(318, RoomType::Quad));
    rooms_matrix.push(Room::new(319, RoomType::Double));
    rooms_matrix.push(Room::new(320, RoomType::Double));
    rooms_matrix.push(Room::new(321, RoomType::Triple));
    rooms_matrix.push(Room::new(322, RoomType::Quad));
    rooms_matrix.push(Room::new(323, RoomType::Double));

    // Fourth floor
    rooms_matrix.push(Room::new(424, RoomType::Quad));
    rooms_matrix.push(Room::new(425, RoomType::Quad));
    rooms_matrix.push(Room::new(426, RoomType::Double));
    rooms_matrix.push(Room::new(427, RoomType::Double));
    rooms_matrix.push(Room::new(428, RoomType::Double));
    rooms_matrix.push(Room::new(429, RoomType::Double));

    rooms_matrix
}



