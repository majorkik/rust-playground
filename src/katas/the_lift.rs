#[cfg(test)]
mod the_lift_test {
    #[derive(Debug)]
    pub enum Direction {
        Up,
        Down,
    }

    impl Direction {
        pub fn flip(&mut self) {
            *self = match *self {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
            };
        }
    }

    #[derive(Debug)]
    pub struct Lift {
        capacity: usize,
        direction: Direction,
        passengers: Vec<u32>,
        history: Vec<u32>,
    }

    impl Lift {
        pub fn default(capacity: u32) -> Self {
            Self {
                capacity: capacity as usize,
                direction: Direction::Up,
                passengers: vec![],
                history: vec![0],
            }
        }

        pub fn update_history(&mut self, floor: u32) {
            if self.history.last() != Some(&floor) {
                self.history.push(floor)
            }
        }

        pub fn has_places(&self) -> bool {
            self.capacity - self.passengers.len() > 0
        }
    }

    fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
        let mut queues = queues.to_owned();
        let mut lift = Lift::default(capacity);

        loop {
            if lift.passengers.is_empty() && queues.iter().all(|i| i.is_empty()) {
                break;
            }

            for index in 0..queues.len() {
                let mut stopped = false;

                let floor = match lift.direction {
                    Direction::Up => index as u32,
                    Direction::Down => (queues.len() - index - 1) as u32,
                };

                if lift.passengers.contains(&floor) {
                    lift.passengers.retain(|pass| *pass != floor);

                    stopped = true;
                }

                let queue = &mut queues[floor as usize];

                if !queue.is_empty() {
                    stopped |= queue.iter().any(|&i| match lift.direction {
                        Direction::Up => i >= floor,
                        Direction::Down => i <= floor,
                    });

                    if queue.contains(&floor) {
                        queue.retain(|pass| *pass != floor);
                    }

                    queue.retain(|pass| {
                        let is_same_direction = match lift.direction {
                            Direction::Up => *pass > floor,
                            Direction::Down => *pass < floor,
                        };

                        if lift.has_places() && is_same_direction {
                            lift.passengers.push(*pass);
                            false
                        } else {
                            true
                        }
                    });
                }

                if stopped {
                    lift.update_history(floor)
                }
            }

            lift.direction.flip();
        }

        lift.update_history(0);

        lift.history
    }

    #[test]
    fn test_up() {
        assert_eq!(the_lift(&[vec![], vec![], vec![5,5,5],vec![],vec![],vec![],vec![]], 5), [0, 2, 5, 0]);
    }
    #[test]
    fn test_down() {
        assert_eq!(the_lift(&[vec![],vec![],vec![1,2],vec![],vec![],vec![],vec![]], 5), [0, 2, 1, 0]);
    }
    #[test]
    fn test_up_and_up() {
        assert_eq!(the_lift(&[vec![],vec![3],vec![4],vec![],vec![5],vec![],vec![]], 5), [0, 1, 2, 3, 4, 5, 0]);
    }
    #[test]
    fn test_down_and_down() {
        assert_eq!(the_lift(&[vec![],vec![0],vec![],vec![],vec![2],vec![3],vec![]], 5), [0, 5, 4, 3, 2, 1, 0]);
    }
}
