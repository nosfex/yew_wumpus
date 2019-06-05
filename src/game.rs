use crate::util::*;

pub struct Game {
    pub arrows: u8,
    pub current_room: u8,
    pub messages: Vec<String>,
    pub wumpus: u8,
    bats: [u8; 2],
    pits: [u8, 2],
}

impl Game {
    fn configure_cave(&mut self) {
            self.messages.push(
              "You've entered a clammy, dark cave, armed with 5 arrows.  You are very cold.".to_string(),
            );
            self.wumpus = js_rand(1, 20);
            self.bats[0] = self.get_empty_room();
            self.bats[1] = self.get_empty_room();
            self.pits[0] = self.get_empty_room();
            self.pits[1] = self.get_empty_room();
            self.warning_messages();
    }

    fn get_empty_room(&self) -> u8 {
        gen_range_avoiding(
          0,
          20,
          vec![
            self.current_room,
            self.wumpus,
            self.bats[0],
            self.bats[1],
            self.pits[0],
            self.pits[1],
          ],
        )
    }

    pub fn warning_messages(&mut self) {
        for adj in &room_exits(self.current_room).unwrap() {
          let t = *adj;
          if self.wumpus == t {
            self
              .messages
              .push("You smell something horrific and rancid.".into());
          } else if self.pits.contains(&t) {
            self
              .messages
              .push("You feel a cold updraft from a nearby cavern.".into());
          } else if self.bats.contains(&t) {
            self
              .messages
              .push("You hear a faint but distinct flapping of wings.".into());
          }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        let mut ret = Self {
          arrows: 5,
          current_room: 1,
          messages: Vec::new(),
          wumpus: 0,
          bats: [0, 0],
          pits: [0, 0],
        };
    ret.configure_cave();
    ret
  }
}
