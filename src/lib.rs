use xplm::{
    command::Command,
    data::{borrowed::DataRef, ArrayRead, ArrayReadWrite, DataRead, DataType, ReadWrite},
    debugln,
    flight_loop::{FlightLoop, LoopState},
    menu::{ActionItem, Menu},
    plugin::{Plugin, PluginInfo},
    xplane_plugin,
};

extern crate xplm;

fn writeable_dataref<T: ?Sized>(name: &str) -> DataRef<T, ReadWrite>
where
    T: DataType,
{
    DataRef::<T>::find(name)
        .expect("dataref to exist")
        .writeable()
        .expect("dataref to be writable")
}

struct DR400Tweaks {
    _callbacks: Vec<FlightLoop>,
    _jf_menu: Menu,
}

impl Plugin for DR400Tweaks {
    type Error = std::convert::Infallible;

    fn start() -> Result<Self, Self::Error> {
        debugln!("[DR400Tweaks] Plugin start starting...");

        let mut callbacks = Vec::new();

        callbacks.push(FlightLoop::new(|state: &mut LoopState| {
            let mut panel_brightness_switches =
                writeable_dataref::<[f32]>("sim/cockpit2/switches/panel_brightness_ratio");
            let mut positions = panel_brightness_switches.as_vec();
            if positions[0] > 0.0 {
                positions[0] = 0.01;
                panel_brightness_switches.set(&positions);
            }

            let mut landing_light_switches =
                writeable_dataref::<[f32]>("sim/cockpit2/switches/landing_lights_switch");
            let mut positions = landing_light_switches.as_vec();
            positions[0] = 1.0;
            landing_light_switches.set(&positions);
            state.deactivate();
        }));
        callbacks.last_mut().unwrap().schedule_after_loops(25);

        if let Ok(start_running) = DataRef::<i32>::find("sim/operation/prefs/startup_running") {
            if start_running.get() == 0 {
                callbacks.push(FlightLoop::new(|state: &mut LoopState| {
                    let mut door_cycle_time =
                        writeable_dataref::<[f32]>("sim/flightmodel2/misc/door_cycle_time");
                    let mut cycles = door_cycle_time.as_vec();
                    cycles[1] = 0.001f32;
                    door_cycle_time.set(&cycles);

                    let mut door_switch =
                        writeable_dataref::<[f32]>("sim/cockpit2/switches/door_open_ratio");
                    let mut switch_pos = door_switch.as_vec();
                    switch_pos[1] = 1f32;
                    door_switch.set(&switch_pos);

                    state.deactivate();
                }));
                callbacks.last_mut().unwrap().schedule_after_loops(1);

                callbacks.push(FlightLoop::new(|state: &mut LoopState| {
                    let mut door_cycle_time =
                        writeable_dataref::<[f32]>("sim/flightmodel2/misc/door_cycle_time");
                    let mut cycles = door_cycle_time.as_vec();
                    cycles[1] = 1f32;
                    door_cycle_time.set(&cycles);

                    Command::find("sim/lights/landing_02_light_off")
                        .expect("stock command to exist")
                        .trigger();
                    Command::find("sim/lights/landing_03_light_off")
                        .expect("stock command to exist")
                        .trigger();
                    state.deactivate();
                }));
                callbacks.last_mut().unwrap().schedule_after_loops(50);
            }
        }

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
        Ok(DR400Tweaks {
            _callbacks: callbacks,
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

xplane_plugin!(DR400Tweaks);
