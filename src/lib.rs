use xplm::{
    command::Command,
    data::{borrowed::DataRef, ArrayRead, ArrayReadWrite},
    debugln,
    flight_loop::{FlightLoop, FlightLoopCallback, LoopState},
    menu::{ActionItem, Menu},
    plugin::{Plugin, PluginInfo},
    xplane_plugin,
};

extern crate xplm;

enum NextStep {
    SetDefaults,
    ResetLeftRightLLSwitch,
    Stop,
}
struct SetLights {
    _next_step: NextStep,
}

impl SetLights {
    fn new() -> SetLights {
        SetLights {
            _next_step: NextStep::SetDefaults,
        }
    }
    fn set_defaults(&mut self) {
        debugln!("[DR400Tweaks] Setting defaults");
        let mut panel_brightness_switches =
            DataRef::<[f32]>::find("sim/cockpit2/switches/panel_brightness_ratio")
                .expect("stock panel brightness dataref to exist")
                .writeable()
                .expect("panel brightness switch dataref to be writeable");
        let mut positions = panel_brightness_switches.as_vec();
        if positions[0] > 0.0 {
            debugln!("[DR400Tweaks] Reducing panel brightness to 0.01");
            positions[0] = 0.01;
            panel_brightness_switches.set(&positions);
        }

        debugln!("[DR400Tweaks] Turning on landing light switch 1");
        let mut landing_light_switches =
            DataRef::<[f32]>::find("sim/cockpit2/switches/landing_lights_switch")
                .expect("stock landing light switches dataref to exist")
                .writeable()
                .expect("landing light switches dataref to be writeable");
        let mut positions = landing_light_switches.as_vec();
        positions[0] = 1.0;
        landing_light_switches.set(&positions);
        self._next_step = match positions[1] < 0.1 {
            true => NextStep::ResetLeftRightLLSwitch,
            false => NextStep::Stop,
        };
    }

    fn reset_landing_lights(&mut self) {
        debugln!("[DR400Tweaks] Turning off left and right landing lights");
        Command::find("sim/lights/landing_02_light_off")
            .expect("stock command to exist")
            .trigger();
        Command::find("sim/lights/landing_03_light_off")
            .expect("stock command to exist")
            .trigger();
        self._next_step = NextStep::Stop;
    }
}

impl FlightLoopCallback for SetLights {
    fn flight_loop(&mut self, state: &mut LoopState) {
        match self._next_step {
            NextStep::SetDefaults => self.set_defaults(),
            NextStep::ResetLeftRightLLSwitch => self.reset_landing_lights(),
            NextStep::Stop => state.deactivate(),
        };
    }
}

struct Callbacks {
    _set_lights: FlightLoop,
    _jf_menu: Menu,
}

impl Plugin for Callbacks {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        debugln!("[DR400Tweaks] Plugin start starting...");
        let mut set_lights = FlightLoop::new(SetLights::new());
        set_lights.schedule_after_loops(25);

        let menu = Menu::new("JustFlight").unwrap();
        menu.add_child(
            ActionItem::new("Show/hide panel", |&_: &_| {
                if let Ok(mut cmd) = Command::find("thranda/popup/1") {
                    cmd.trigger();
                }
            })
            .unwrap(),
        );
        menu.add_to_plugins_menu();
        Ok(Callbacks {
            _set_lights: set_lights,
            _jf_menu: menu,
        })
    }

    fn info(&self) -> PluginInfo {
        let ts = env!("VERGEN_BUILD_TIMESTAMP")
            .get(0..16)
            .unwrap_or("1970-01-01T00:00");
        let debug = match env!("VERGEN_CARGO_DEBUG") {
            "true" => "debug",
            _ => "release",
        };
        let opt_level = env!("VERGEN_CARGO_OPT_LEVEL");

        PluginInfo {
            name: String::from("DR400 Tweaks"),
            signature: String::from("com.jdeeth.dr400tweaks"),
            description: format!(
                "Fixes a couple of lighting issues. Compiled {ts}Z, {debug} opt level {opt_level}"
            ),
        }
    }
}

xplane_plugin!(Callbacks);
