#[macro_use]
extern crate yew;

mod components;
mod game;
mod util;

use self::components::{controls::Controls,
                        stats::Stats,
                        messages::Messages,
                        game::Game,
                        util::*};
use yew::prelude::*;

fn room_exits(id: u8) -> Option<[u8; 3]> {
  match id {
    1 => Some([2, 5, 8]),
    2 => Some([1, 3, 10]),
    3 => Some([2, 4, 12]),
    4 => Some([3, 5, 14]),
    5 => Some([1, 4, 6]),
    6 => Some([5, 7, 15]),
    7 => Some([6, 8, 17]),
    8 => Some([1, 7, 11]),
    9 => Some([10, 12, 19]),
    10 => Some([2, 9, 11]),
    11 => Some([8, 10, 20]),
    12 => Some([3, 9, 13]),
    13 => Some([12, 14, 18]),
    14 => Some([4, 13, 15]),
    15 => Some([6, 14, 16]),
    16 => Some([15, 17, 18]),
    17 => Some([7, 16, 20]),
    18 => Some([13, 16, 19]),
    19 => Some([9, 18, 20]),
    20 => Some([11, 17, 19]),
    _ => None
  }
}

pub enum Model {
    Waiting(String),
    Playing(Game),
}

#[derive(Debug, Clone)]
pub enum Msg {
    SwitchRoom(u8),
}
impl Component for Model {

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let mut rng = thread_rng();
        let mut ret = Model::default();
        ret.configure_cave();
        ret
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SwitchRoom(target) => {
                self.current_room = target;
                self.messages.push(format!("Moved to room {}", target));
                self.warning_messages();
                true
            }
        }
    }
}

impl Model {
    fn configure_cave(&mut self) {
        self.messages.push("You've entered a clammy, dark cave, armed with 5 arrows.  You are very cold.".to_string());

        self.wumpus = js_rand(1, 20);
        self.bats[0] = self.get_empty_room();
        self.bats[1] = self.get_empty_room();
        self.pits[0] = self.get_empty_room();
        self.pits[1] = self.get_empty_room();
        self.warning_messages();
    }

    fn get_empty_room(&self) -> u8 {
        gen_range_avoiding(0,20, vec![self.current_room,
                                        self.wumpus,
                                        self.bats[0],
                                        self.bats[1],
                                        self.pits[0],
                                        self.pits[1]],)

    }

    fn warning_messages(&mut self) {
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

impl Default for Model {
    fn default() -> Self {
        Model::Waiting("New Game!".into())
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="hunt",>
                <div class="header",>{"Hunt the Wumpus"}</div>
                <div class="window",>
                    <Stats: arrows=self.arrows, current_room=self.current_room,/>
                    <Controls: exits=room_exits(self.current_room).unwrap(), onsignal=|msg| msg,/>
                </div>

                <Messages: messages=&self.messages,/>
            </div>

        }
    }
}
