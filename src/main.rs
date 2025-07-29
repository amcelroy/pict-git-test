extern crate alloc;

use core::time::Duration;
use pictorus_blocks::{SinewaveBlock, TrianglewaveBlock};
use pictorus_internal::loggers::linux_logger::LinuxLogger;
use pictorus_internal::loggers::Logger;
use pictorus_internal::timing::{RunTime, Timing};
use pictorus_internal::utils::{
    custom_panic_handler, get_diagram_params, get_pictorus_vars, initialize_logging, load_param,
    DiagramParams, PictorusError, PictorusVars,
};
use pictorus_linux::{
    create_clock_protocol, create_delay_protocol, StandardClock, StdDelayProtocol,
};
use pictorus_traits::{Context as CorelibContext, GeneratorBlock};

pub fn compile_info() -> &'static str {
    return "git_test_6888d8981b7c297dc266691a version : compiled 07/29/2025 - 15:22:56";
}

#[derive(Debug, Clone)]
pub enum State {
    Main66979State,
}

pub struct Main66979State {
    last_time_s: f64,
    sinewave1_66975_param: <SinewaveBlock<f64> as GeneratorBlock>::Parameters,
    sinewave1_66975: SinewaveBlock<f64>,
    trianglewave1_66977_param: <TrianglewaveBlock<f64> as GeneratorBlock>::Parameters,
    trianglewave1_66977: TrianglewaveBlock<f64>,
}

impl Main66979State {
    pub fn new(_context: &Context) -> Self {
        let pictorus_vars = get_pictorus_vars();
        let diagram_params = get_diagram_params(&pictorus_vars);

        let sinewave1_66975_amplitude =
            load_param::<f64>(&"sinewave1_66975", &"amplitude", 1.000000, &diagram_params);
        let sinewave1_66975_frequency =
            load_param::<f64>(&"sinewave1_66975", &"frequency", 1.000000, &diagram_params);
        let sinewave1_66975_phase =
            load_param::<f64>(&"sinewave1_66975", &"phase", 0.000000, &diagram_params);
        let sinewave1_66975_bias =
            load_param::<f64>(&"sinewave1_66975", &"bias", 0.000000, &diagram_params);

        // Sinewave1
        let sinewave1_66975_param = <SinewaveBlock<f64> as GeneratorBlock>::Parameters::new(
            sinewave1_66975_amplitude,
            sinewave1_66975_frequency,
            sinewave1_66975_phase,
            sinewave1_66975_bias,
        );
        let sinewave1_66975 = SinewaveBlock::default();

        let trianglewave1_66977_amplitude = load_param::<f64>(
            &"trianglewave1_66977",
            &"amplitude",
            1.000000,
            &diagram_params,
        );
        let trianglewave1_66977_frequency = load_param::<f64>(
            &"trianglewave1_66977",
            &"frequency",
            1.000000,
            &diagram_params,
        );
        let trianglewave1_66977_phase =
            load_param::<f64>(&"trianglewave1_66977", &"phase", 0.000000, &diagram_params);
        let trianglewave1_66977_bias =
            load_param::<f64>(&"trianglewave1_66977", &"bias", 0.000000, &diagram_params);

        // Trianglewave1
        let trianglewave1_66977_param = <TrianglewaveBlock<f64> as GeneratorBlock>::Parameters::new(
            trianglewave1_66977_amplitude,
            trianglewave1_66977_frequency,
            trianglewave1_66977_phase,
            trianglewave1_66977_bias,
        );
        let trianglewave1_66977 = TrianglewaveBlock::default();

        Main66979State {
            last_time_s: -1.0,
            sinewave1_66975_param,
            sinewave1_66975,
            trianglewave1_66977_param,
            trianglewave1_66977,
        }
    }

    pub fn run(&mut self, context: &mut Context) {
        let app_time_s = context.app_time_s();
        let runtime_ctx = context.get_runtime_context();

        if self.last_time_s == -1.0 {
            self.last_time_s = app_time_s;
        }
        let timestep_s: f64 = app_time_s - self.last_time_s;

        log::debug!(
            "-- State::Main66979State iteration. Time: {}s (dt: {}s) ",
            app_time_s,
            timestep_s
        );

        // Sinewave1
        let sinewave1_66975_0 = self
            .sinewave1_66975
            .generate(&self.sinewave1_66975_param, &runtime_ctx);
        context.log_data.sinewave1_66975_0(sinewave1_66975_0);
        // Trianglewave1
        let trianglewave1_66977_0 = self
            .trianglewave1_66977
            .generate(&self.trianglewave1_66977_param, &runtime_ctx);
        context
            .log_data
            .trianglewave1_66977_0(trianglewave1_66977_0);

        self.last_time_s = app_time_s;
    }

    pub fn post_run(&mut self) {}
}

pub struct StateManager {
    pub current_state: State,
    pub main66979_state: Main66979State,
}

impl StateManager {
    pub fn run(&mut self, context: &mut Context) {
        match self.current_state {
            State::Main66979State => self.main66979_state.run(context),
        };
    }
}

#[derive(Debug, serde::Serialize, Default)]
struct LogData {
    pub state_id: Option<&'static str>,
    pub timestamp: Option<f64>,
    pub app_time_us: Option<u64>,
    #[serde(rename = "sinewave1_66975.0")]
    pub sinewave1_66975_0: Option<f64>,
    #[serde(rename = "trianglewave1_66977.0")]
    pub trianglewave1_66977_0: Option<f64>,
}

impl LogData {
    pub fn state_id(&mut self, value: &'static str) {
        self.state_id = Some(value);
    }

    pub fn timestamp(&mut self, value: f64) {
        self.timestamp = Some(value);
    }

    pub fn app_time_us(&mut self, value: u64) {
        self.app_time_us = Some(value);
    }

    pub fn sinewave1_66975_0(&mut self, value: f64) {
        self.sinewave1_66975_0 = Some(value);
    }

    pub fn trianglewave1_66977_0(&mut self, value: f64) {
        self.trianglewave1_66977_0 = Some(value);
    }
}

pub struct IoManager {}

impl IoManager {
    pub fn new(
        diagram_params: &DiagramParams,
    ) -> Result<(Self, Timing<StandardClock, StdDelayProtocol>), PictorusError> {
        let app_run_time_s = load_param::<f64>(
            &String::from("app"),
            &String::from("run_time_s"),
            10.0,
            &diagram_params,
        );
        let app_hertz = load_param::<f64>(
            &String::from("app"),
            &String::from("hertz"),
            10.0,
            &diagram_params,
        );
        let use_realtime = true;
        let app_clock = create_clock_protocol();
        let app_delay = create_delay_protocol();
        let timing = Timing::new(
            RunTime::from_f64_seconds(app_run_time_s),
            app_hertz,
            use_realtime,
            app_clock,
            app_delay,
        );

        let io_manager = IoManager {};
        Ok((io_manager, timing))
    }

    pub fn flush_inputs(&mut self) {}
}

pub struct AppInterface {
    state_manager: StateManager,
    data_logger: LinuxLogger,
    context: Context,
}

impl AppInterface {
    pub fn new(context: Context, pictorus_vars: &PictorusVars) -> Self {
        let data_logger_path =
            std::path::PathBuf::from(&pictorus_vars.run_path).join("diagram_output.csv");
        let data_log_period = if pictorus_vars.data_log_rate_hz > 0.0 {
            Duration::from_secs_f64(1.0 / pictorus_vars.data_log_rate_hz)
        } else {
            Duration::ZERO
        };
        let data_logger = LinuxLogger::new(
            Duration::from_micros(1000),
            &pictorus_vars.publish_socket,
            data_log_period,
            data_logger_path,
        );

        let state_manager = StateManager {
            current_state: State::Main66979State,
            main66979_state: Main66979State::new(&context),
        };

        Self {
            state_manager,
            data_logger,
            context,
        }
    }

    pub fn update(&mut self) {
        self.state_manager.run(&mut self.context);

        let logged_state_id = match self.state_manager.current_state {
            State::Main66979State => "main66979_state",
        };
        self.context.log_data.state_id(logged_state_id);

        let app_time_s = self.context.app_time_s();
        self.context.log_data.timestamp(app_time_s);
        let app_time_us = self.context.app_time_us();
        self.context.log_data.app_time_us(app_time_us);

        self.data_logger
            .log(&self.context.log_data, self.context.time());

        self.context.io_manager.flush_inputs();
    }
}

pub struct Context {
    log_data: LogData,
    io_manager: IoManager,
    runtime_context: pictorus_internal::RuntimeContext,
}

impl Context {
    pub fn app_time_s(&self) -> f64 {
        self.runtime_context.app_time_s()
    }

    pub fn app_time_us(&self) -> u64 {
        self.runtime_context.app_time_us()
    }

    pub fn time(&self) -> Duration {
        self.runtime_context.time()
    }

    pub fn get_runtime_context(&self) -> pictorus_internal::RuntimeContext {
        self.runtime_context
    }

    pub fn update_app_time(&mut self, app_time_us: u64) {
        self.runtime_context.update_app_time(app_time_us);
    }
}

fn main() -> std::process::ExitCode {
    use std::sync::Arc;

    let pictorus_vars = get_pictorus_vars();
    let diagram_params = get_diagram_params(&pictorus_vars);

    let og_panic = std::panic::take_hook();
    let run_path_clone = pictorus_vars.run_path.clone();
    std::panic::set_hook(Box::new(move |panic_info| {
        custom_panic_handler(panic_info, &run_path_clone);
        og_panic(panic_info)
    }));

    initialize_logging();
    log::info!("{}", compile_info());

    let log_data = LogData::default();

    let (io_manager, mut timing) =
        IoManager::new(&diagram_params).expect("Unable to initialize IoManager!");
    let context = Context {
        log_data,
        io_manager,
        runtime_context: pictorus_internal::RuntimeContext::new(100000),
    };

    let mut app_interface = AppInterface::new(context, &pictorus_vars);

    let interrupt = Arc::new(std::sync::atomic::AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&interrupt)).unwrap();
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&interrupt)).unwrap();

    while timing.should_run(app_interface.context.app_time_us())
        && !interrupt.load(std::sync::atomic::Ordering::Relaxed)
    {
        app_interface.update();
        app_interface
            .context
            .update_app_time(timing.update(app_interface.context.app_time_us()));
    }

    log::info!("Exiting git_test_6888d8981b7c297dc266691a.");
    std::process::ExitCode::SUCCESS
}
