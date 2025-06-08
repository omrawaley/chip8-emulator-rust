pub enum KeyState {
    Pressed,
    Released,
}

pub enum Key {
    One(KeyState),
    Two(KeyState),
    Three(KeyState),
    C(KeyState),
    Four(KeyState),
    Five(KeyState),
    Six(KeyState),
    D(KeyState),
    Seven(KeyState),
    Eight(KeyState),
    Nine(KeyState),
    E(KeyState),
    A(KeyState),
    Zero(KeyState),
    B(KeyState),
    F(KeyState),
}

pub struct Keypad {
    pub keys: [Key; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [
                Key::One(KeyState::Released),
                Key::Two(KeyState::Released),
                Key::Three(KeyState::Released),
                Key::C(KeyState::Released),
                Key::Four(KeyState::Released),
                Key::Five(KeyState::Released),
                Key::Six(KeyState::Released),
                Key::D(KeyState::Released),
                Key::Seven(KeyState::Released),
                Key::Eight(KeyState::Released),
                Key::Nine(KeyState::Released),
                Key::E(KeyState::Released),
                Key::A(KeyState::Released),
                Key::Zero(KeyState::Released),
                Key::B(KeyState::Released),
                Key::F(KeyState::Released),
            ],
        }
    }

    pub fn set_key(&mut self, key: Key) {
        let index = self.key_to_index(&key);
        self.keys[index] = key;
    }

    fn key_to_index(&self, key: &Key) -> usize {
        match key {
            Key::One(_) => 1,
            Key::Two(_) => 2,
            Key::Three(_) => 3,
            Key::C(_) => 12,
            Key::Four(_) => 4,
            Key::Five(_) => 5,
            Key::Six(_) => 6,
            Key::D(_) => 13,
            Key::Seven(_) => 7,
            Key::Eight(_) => 8,
            Key::Nine(_) => 9,
            Key::E(_) => 14,
            Key::A(_) => 10,
            Key::Zero(_) => 0,
            Key::B(_) => 11,
            Key::F(_) => 15,
        }
    }

    pub fn is_key_pressed(&self, key: &Key) -> bool {
        let index = self.key_to_index(&key);
        matches!(self.keys[index], 
            Key::One(KeyState::Pressed) |
            Key::Two(KeyState::Pressed) |
            Key::Three(KeyState::Pressed) |
            Key::C(KeyState::Pressed) |
            Key::Four(KeyState::Pressed) |
            Key::Five(KeyState::Pressed) |
            Key::Six(KeyState::Pressed) |
            Key::D(KeyState::Pressed) |
            Key::Seven(KeyState::Pressed) |
            Key::Eight(KeyState::Pressed) |
            Key::Nine(KeyState::Pressed) |
            Key::E(KeyState::Pressed) |
            Key::A(KeyState::Pressed) |
            Key::Zero(KeyState::Pressed) |
            Key::B(KeyState::Pressed) |
            Key::F(KeyState::Pressed)
        )
    }
}