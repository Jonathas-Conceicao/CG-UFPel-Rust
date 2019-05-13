use super::Command;

#[derive(Clone, Debug, Default)]
pub(super) struct Animation {
    pub is_running: bool,
    pub command_pool: Vec<(Command, f32)>,
}

impl Animation {
    pub fn start(&mut self, cmds: Vec<(Command, f32)>) {
        if self.is_running {
            return;
        }
        self.is_running = true;
        self.command_pool = cmds;
    }

    pub fn step(&mut self, delta_time: f32) -> Vec<(Command, f32)> {
        if self.command_pool.len() < 1 {
            self.stop();
            return Vec::default();
        }
        let mut delta_time = delta_time;
        let mut vec = Vec::default();
        for (c, t) in &mut self.command_pool {
            if delta_time <= *t {
                vec.push((*c, delta_time));
                *t -= delta_time;
                break;
            }
            vec.push((*c, *t));
            delta_time -= *t;
            *t = 0.;
        }
        self.consume();
        return vec;
    }

    fn consume(&mut self) {
        self.command_pool = self
            .command_pool
            .iter()
            .cloned()
            .filter(|(_, t)| *t > 0.)
            .collect();
    }

    // Called automatically when step reach end of command pool
    fn stop(&mut self) {
        self.is_running = false;
        self.command_pool = Vec::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn steps() {
        let mut ani = Animation::default();
        let cmds = vec![
            (Command::ScaleU, 0.5),
            (Command::SlideXF, 0.5),
            (Command::SlideZB, 0.5),
        ];
        ani.start(cmds);
        let ret = ani.step(0.1);
        assert_eq!(ret.len(), 1);
        let ret = ani.step(0.3);
        assert_eq!(ret.len(), 1);
        let ret = ani.step(0.7);
        assert_eq!(ret.len(), 3);
        let ret = ani.step(0.7);
        assert_eq!(ret.len(), 1);
    }
}
